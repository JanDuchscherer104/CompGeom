
filename = 'data/polygon.txt';

vertices = readmatrix(filename);

fprintf('Read %i vertices\n', size(vertices, 1));

% disp('Polygon vertices:');
% disp(vertices);

[center, radius] = max_inscribed_circle(vertices);

fprintf('Found solution:\n');
fprintf('Center: (%f, %f)\n', center(1), center(2));
fprintf('Radius: %f\n', radius);

plot_result(vertices, center, radius);

function [center, radius] = max_inscribed_circle(vertices)
    % Normalize the polygon
    min_coords = min(vertices);
    max_coords = max(vertices);
    scale = max(max_coords - min_coords);
    normalized_vertices = (vertices - min_coords) / scale;
    
    n = size(normalized_vertices, 1);
    
    % Zielfunktion: maximize radius, ignore x and y
    f = [0; 0; -1];
    
    % Initialize constraint arrays
    A = [];
    b = [];
    
    for i = 1:n
        j = mod(i, n) + 1;  % Next vertex (wrap around)
        
        edge = normalized_vertices(j, :) - normalized_vertices(i, :);
        normal = [-edge(2), edge(1)];
        normal = normal / norm(normal);
        
        % Ensure the normal points inward
        midpoint = (normalized_vertices(i, :) + normalized_vertices(j, :)) / 2;
        test_point = midpoint + normal * 0.01;
        if ~inpolygon(test_point(1), test_point(2), normalized_vertices(:,1), normalized_vertices(:,2))
            normal = -normal;
        end
        
        p = normalized_vertices(i,:);

        % Add constraint: normal * (x - p) >= r
        A = [A; -normal, 1];
        b = [b; -normal * p'];
    end

    % verify_normals(vertices, A);
    
    % Bounding box constraints
    lb = [0; 0; 0];
    ub = [1; 1; 0.5];  % Max radius can't be more than half the unit square
    
    options = optimoptions('linprog', 'Display', 'off');
    [solution, ~, exitflag, output] = linprog(f, A, b, [], [], lb, ub, options);
    
    if exitflag > 0
        % Denormalize the solution
        center = solution(1:2) * scale + min_coords;
        radius = solution(3) * scale;
    else
        fprintf('Linear programming problem could not be solved.\n');
        fprintf('Exit flag: %d\n', exitflag);
        fprintf('Output message: %s\n', output.message);
        
        % Debug information
        disp('Constraint matrix A:');
        disp(A);
        disp('Constraint vector b:');
        disp(b);
        
        error('Failed to find a solution.');
    end
end

function plot_result(vertices, center, radius)
    figure;
    hold on;
    
    % Plot the polygon
    plot([vertices(:,1); vertices(1,1)], [vertices(:,2); vertices(1,2)], 'b-', 'LineWidth', 2);
    
    % Plot the inscribed circle
    theta = linspace(0, 2*pi, 100);
    x = center(1) + radius * cos(theta);
    y = center(2) + radius * sin(theta);
    plot(x, y, 'r-', 'LineWidth', 2);
    
    % Plot the center point
    plot(center(1), center(2), 'ro', 'MarkerSize', 10, 'MarkerFaceColor', 'r');
    
    axis equal;
    title('Polygon with Maximum Inscribed Circle');
    legend('Polygon', 'Inscribed Circle', 'Circle Center');
    hold off;
end

function verify_normals(vertices, A)
    n = size(vertices, 1);
    
    figure;
    hold on;
    
    % Plot the polygon
    plot([vertices(:,1); vertices(1,1)], [vertices(:,2); vertices(1,2)], 'b-', 'LineWidth', 2);
    
    for i = 1:n
        if norm(A(i, 1:2)) < eps
            continue; % Skip zero-length edges
        end
        
        % Extract the normal from the constraint matrix A
        normal = A(i, 1:2);
        
        % Find the corresponding edge
        j = mod(i, n) + 1;  % Next vertex (wrap around)
        
        % Midpoint of the edge
        midpoint = (vertices(i, :) + vertices(j, :)) / 2;
        
        % Test point slightly in the direction of the normal
        test_point = midpoint + normal * 0.1;
        
        % Check if the test point is inside the polygon
        if ~inpolygon(test_point(1), test_point(2), vertices(:,1), vertices(:,2))
            normal = -normal;  % Reverse the normal
        end
        
        % Plot the normal vector
        quiver(midpoint(1), midpoint(2), normal(1), normal(2), 0.5, 'r', 'LineWidth', 1, 'MaxHeadSize', 2);
    end
    
    axis equal;
    title('Polygon with Normal Vectors');
    hold off;
end
