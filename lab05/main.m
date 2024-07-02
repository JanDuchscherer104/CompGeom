
filename = 'data/polygon.txt';

vertices = readmatrix(filename);

fprintf('Read %i vertices\n', size(vertices, 1));

% disp('Polygon vertices:');
% disp(vertices);

[center, radius] = max_inscribed_circle(vertices);

fprintf('Found solution:\n');
fprintf('Center: (%f, %f)\n', center(1), center(2));
fprintf('Radius: %f\n', radius);

function [center, radius] = max_inscribed_circle(vertices)
    n = size(vertices, 1);
    
    % Function coefficients: maximize radius, ignore x and y
    c = [0; 0; -1];
    
    A = zeros(n, 3);
    b = zeros(n, 1);
    
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
        normal = normal / norm(normal);
        
        % Add the constraint: normal * (x, y) - r >= normal * vertex_i
        A(i, :) = [normal, -1];
        b(i) = dot(normal, vertices(i, :));
    end
    
    % bounding box as lower and upper bounds
    max_x = max(vertices(:,1));
    min_x = min(vertices(:,1));
    max_y = max(vertices(:,2));
    min_y = min(vertices(:,2));
    max_radius = min(max_x - min_x, max_y - min_y) / 2;
    
    lb = [min_x; min_y; 0];
    ub = [max_x; max_y; max_radius];
    
    options = optimoptions('linprog', 'Display', 'off');
    [solution, ~, exitflag, output] = linprog(c, A, b, [], [], lb, ub, options);
    
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