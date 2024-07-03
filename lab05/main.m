
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
    n = size(vertices, 1);
    
    % Zielfunktion: maximize radius, ignore x and y
    f = [0; 0; 1];
    
    % Initialize constraint arrays
    A = [];
    b = [];
    
    for i = 1:n
        j = mod(i, n) + 1;  % Next vertex (wrap around)
        
        % edge vector
        edge = vertices(j, :) - vertices(i, :);
        
        % normal vector
        normal = [-edge(2), edge(1)];
        
        % normalize the normal vector
        normal_length = norm(normal);
        if normal_length < eps
            warning('Edge %d has zero length. Skipping this constraint.', i);
            continue;
        end
        normal = normal / normal_length;
        
        % Ensure the normal points inward
        midpoint = (vertices(i, :) + vertices(j, :)) / 2;
        test_point = midpoint + normal * 0.1;
        if ~inpolygon(test_point(1), test_point(2), vertices(:,1), vertices(:,2))
            normal = -normal;
        end
        
        n_x = normal(1);
        n_y = normal(2);
        p_x = vertices(i,1);
        p_y = vertices(i,2);

        % distance >= r
        % distance = dot(normal, (center-point))
        % n_x * (x - p_x) + n_y * (y - p_y) >= r
        % n_x * x + n_y * y - r <= n_x*p_x + n_y * p_y

        b_i = n_x * p_x + n_y * p_y;

        fprintf('%f * x + %f * y - r <= %.1f\n', n_x, n_y, b_i);

        A = [A; n_x, n_y, -1];
        b = [b; b_i];

    end

    disp(A);
    disp(b);

    verify_normals(vertices, A);
    
    % bounding box as lower and upper bounds
    max_x = max(vertices(:,1));
    min_x = min(vertices(:,1));
    max_y = max(vertices(:,2));
    min_y = min(vertices(:,2));
    max_radius = min(max_x - min_x, max_y - min_y) / 2;

    fprintf("Bounding box: (%f, %f) - (%f, %f)\n", min_x, min_y, max_x, max_y);

    
    lb = [min_x; min_y; 0];
    ub = [max_x; max_y; max_radius];
    
    options = optimoptions('linprog', 'Display', 'off');
    [solution, ~, exitflag, output] = linprog(f, A, b, [], [], lb, ub, options);
    
    if exitflag > 0
        center = solution(1:2);
        radius = solution(3);
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
