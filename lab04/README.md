# Exercise 04
Installieren Sie das Programm qhull, erzeugen Sie zufällige Punktemengen und berechnen Sie mit qhull konvexe Hüllen, auch in höheren Dimensionen (qhull bringt ein Werkzeug zur Erzeugung von Punktmengen mit). Plotten Sie die Zeiten für zunehmende Punktanzahlen bei unterschiedlichen Dimensionen (2-8). Versuchen Sie, die Ausgaben von qhull bei "geschwätzigster" Einstellung nachzuvollziehen, zu verstehen und ggf. mit Inhalten dieser Lehrveranstaltung in Einklang zu bringen.

## Excercise 04.1
The solution is documented in the Jupiter notebook [main.ipynb](main.ipynb).

## Exercise 04.2
Ermitteln Sie für ein vorgegebenes konvexes Polygon ( polygon.txt und testpolygon.txt) mit Linear Programming den grössten einbeschreibbaren Kreis. Verwenden Sie zur Formulierung und Lösung des Problems entweder (vorzugsweise) MATLAB oder einen Online-Löser aus dem Internet.

### Result
**Center**: (472.570536, 480.865619)  
**Radius**: 438.592170

![](result.png)


## Exercise 04.2
To comprehend the output I generated a small example using `rbox 8 D2` to generate 8 random points in 2D. The output is saved in [points.txt](points.txt). Furthermore, I created a visualization of the points in [Geogebra](https://www.geogebra.org/calculator/tsgypseu).  
Then I ran `qhull s i n p FA FS Fx` with the following parameters:
```
s    - summary of results (default)
i    - vertices incident to each facet
n    - normals with offsets
p    - vertex coordinates (if 'Qc', includes coplanar points)
        if 'v', Voronoi vertices
FA   - report total area and volume
FS   - total area and volume
Fx   - extreme points (convex hull vertices)
```

### Output and Interpretation
```python
Convex hull of 8 points in 2-d:

  Number of vertices: 6 # 6 vertices that form the convex hull.
  Number of facets: 6  # 6 facets (edges in 2D) that form the convex hull.

Statistics for: rbox 8 D2 | qhull s i n p FA FS Fx

  Number of points processed: 6 # Only 6 out of the 8 points were needed to form the convex hull.
  Number of hyperplanes created: 9
  # 9 lines (hyperplanes) were created during the computation.
  # Hyperplanes are created at each step to form the edges of the convex hull.
  # Each recursive step adds new lines, resulting in 9 lines in total.

  Number of distance tests for qhull: 29
  # 29 distance tests were performed to determine point positions relative to the lines.
  # Each point's distance from the current segment is calculated to find the farthest point.
  # These tests are repeated for each new segment.

  CPU seconds to compute hull (after input): 4.2e-05
  Total facet area:   2.6784877 # Total area of the convex hull.
  Total volume:       0.43681561 # Total volume of the convex hull.
6
# Vertices incident to each facet:
0 7
6 1
3 6
7 3
5 0
1 5

# Specifies the dimensions of the matrix below
3
6
# Normal vectors with offsets for each facet. These are needed to calculate the distance of a point to the facet
-0.931582294208225 -0.3635305064471757 -0.1165460722690878
0.9981922385397314 0.06010203756146724 -0.4605316988066304
0.773162386160488 -0.6342081082946042 -0.4688257875529295
0.07535513056073408 -0.9971567601426442 -0.4232351366942544
-0.02096711567724071 0.9997801658665656 -0.4671991948163758
0.6170688579530165 0.7869091590168209 -0.4010281789589362

# Specifies the dimensions of the matrix below
2
6
# Vertex coordinates that define the convex hull
-0.3049645440699202 0.4609062908784545
0.4520232742205479 0.1551623741678543
0.2752779417441021 -0.4036391744424023
0.05256550298311324 0.4684043130543123
0.4712819251895713 -0.1646909403285858
0.03936327345609969 -0.4214672445519522

0 
# Volume and area summary (FA and FS options)
2 2.67848773976793 0.4368156142581473

# Number of extreme points
6
# List of the indices of the vertices that define the convex hull
0
7
3
6
1
5
```
