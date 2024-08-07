# Computational Geometry (Exercise 1 and 3)

## Usage of the tool
The program can be run with `cargo run <args>`, where `<args>` are the following:
- `benchmark <filename>/all`
  - `all` will run the benchmark Sweep Line on all files in the `data` directory
  - `<filename>` will run the benchmark Sweep Line on the file with the given name
  - `--brute-force` will run the benchmark with the brute force algorithm
  - `--sweep-line` will run the benchmark with the sweep line algorithm
  - `--external` will run the benchmark with an external implementation of the [geo library](https://docs.rs/geo/latest/geo/)
- `analyze <filename>`
  - uses the brute force algorithm to output additional information about illegal line segments and intersection types

where `<file>` is the path to the file containing the line segments (e.g. `data/s_1000_1.dat`).

### Example commands
- `cargo run benchmark all`
- `cargo run benchmark data/s_1000_1.dat`
- `cargo run analyze data/s_1000_1.dat`

## Exercise 01
In dem Tar-File 'strecken.tgz' (s.u.) befinden sich Dateien mit jeweils 4 Koordinaten pro Zeile. Diese stellen jeweils die x- und y-Koordinaten eines Start- bzw. Endpunkts einer Strecke dar. Lesen Sie jeweils eine Datei ein und ermitteln Sie die Anzahl der sich schneidenden (d.h. mindestens ein gemeinsamer Punkt) Strecken, indem Sie jedes Paar von Strecken gegeneinander testen. Messen Sie die pro Datei aufgewendete Zeit. Begründen Sie nachvollziehbar, warum die Anzahl der von Ihrem Programm jeweils gefundenen Schnittpunkte korrekt ist.

### Results

#### Custom implementation (Brute-force)
| File                 | # Lines    | # Intersections | CPU Time (ms)   | Memory (kB)     |
| -------------------- | ---------- | --------------- | --------------- | --------------- |
| data/s_1000_1.dat    | 1001       | 11              | 60              | 92              |
| data/s_1000_10.dat   | 994        | 796             | 61              | 288             |
| data/s_10000_1.dat   | 10001      | 733             | 6053            | 176             |
| data/s_100000_1.dat  | 100001     | 77135           | 602864          | 16276           |


#### External implementation (Brute-force with Geo library)
| File                 | # Lines    | # Intersections | CPU Time (ms)   | Memory (kB)     |
| -------------------- | ---------- | --------------- | --------------- | --------------- |
| data/s_1000_1.dat    | 1001       | 11              | 30              | 112             |
| data/s_1000_10.dat   | 994        | 796             | 31              | 184             |
| data/s_10000_1.dat   | 10001      | 733             | 3050            | 72              |
| data/s_100000_1.dat  | 100001     | 77135           | 313160          | 16276           |

### Justification
#### Algorithm
- Simple but reliable Brute-force approach to compare all pairs of lines
- For each pair of lines, the intersection is calculated
#### Tests
- Tests covering many edge cases
- Tests are comparing the results with the well-established library geo
#### Weaknesses
- The algorithm is not optimal and has a time complexity of O(n^2)
- Floating point arithmetic can lead to errors

## Exercise 03
Implementieren Sie unter Zuhilfenahme der Funktionalität aus Aufgabe 1 zur Berechnung von Schnittpunkten zwischen Linien einen Sweep Line Algorithmus und vergleichen Sie die erzielten Laufzeiten. Verwenden Sie für die Laufzeitvergleiche neben den Daten aus der ersten Aufgabe die Daten aus s_1000_10.dat. Vergleichen Sie ebenso die Laufzeiten für die Files s_1000_1.dat und s_1000_10.dat (s.u.) .

### Results
| File                 | # Lines    | # Intersections | CPU Time (ms)   | Memory (kB)     |
| -------------------- | ---------- | --------------- | --------------- | --------------- |
| data/s_1000_1.dat    | 1001       | 4               | 5               | 132             |
| data/s_1000_10.dat   | 994        | 796             | 27              | 600             |
| data/s_10000_1.dat   | 10001      | 709             | 191             | 104             |
| data/s_100000_1.dat  | 100001     | 63344           | 15058           | 13052           |


#### Interpretation of results
##### Validity
- The file `s_1000_1.dat` actually contains 11 intersections, however 4 of them are overlapping lines and 3 of them are touching lines. As such, the implementation works as expected.
- The file `s_1000_10.dat` contains 796 intersections, which is the expected result.
- The file `s_10000_1.dat` contains 733 intersections. There are again 4 overlapping and 3 touching lines. The remaining difference might be due to vertical, zero-length lines. Furthermore, end and intersection events with the same x-coordinate are filtered out.
- The file `s_100000_1.dat` actually contains 77135 intersections. There are again 4 overlapping and 3 touching lines. The remaining difference might be due to vertical, zero-length lines. Furthermore, 15636 end and intersection events with the same x-coordinate are filtered out.

##### Performance
- The Sweep Line algorithm is significantly faster than the brute-force algorithm, even though the implementation is not optimal.
- Bentley Ottman can achieve a time complexity of O((n + k) log n), when balanced binary search trees are used for the event queue and the sweep line structure
- I used a vector for the sweep-line structure and I resort the line segments after each event. Given that there are 2n+k events in total and [rusts sort implementation](https://doc.rust-lang.org/std/primitive.slice.html#method.sort) is based on timsort, which has a time complexity of O(n log n), the time complexity of the implementation increases to O((n + k) n log n)
- Rusts sort implementation favors nearly sorted arrays, which is the case for most of the line segments after each event.
- The implementation ignores segments that don't meet the requirements, which can lead to a performance increase

### Implementation details
- The implementation is based on the Bentley Ottmann Algorithm
- Following requirements are needed to ensure proper functionality
    - No vertical lines
    - No zero-length lines
    - X-coordinates of intersections and the endpoints of the line segments are unique
    - No overlapping lines
    - No touching lines
- if the above requirements are not met, the program can be configured to either ignore the lines or to panic
- Sweep Line data structure recomputes sorting of the line segments after each event by inserting x-coordinates of the event (x+eps in the case of an intersection). Notably, this is not very efficient and can also lead to errors.
- The implementation efficiency can be improved by using a more efficient data structure for the line segments (e.g. AVL tree)
