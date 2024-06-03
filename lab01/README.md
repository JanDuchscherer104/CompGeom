## Excercise 01
In dem Tar-File 'strecken.tgz' (s.u.) befinden sich Dateien mit jeweils 4 Koordinaten pro Zeile. Diese stellen jeweils die x- und y-Koordinaten eines Start- bzw. Endpunkts einer Strecke dar. Lesen Sie jeweils eine Datei ein und ermitteln Sie die Anzahl der sich schneidenden (d.h. mindestens ein gemeinsamer Punkt) Strecken, indem Sie jedes Paar von Strecken gegeneinander testen. Messen Sie die pro Datei aufgewendete Zeit. Begründen Sie nachvollziehbar, warum die Anzahl der von Ihrem Programm jeweils gefundenen Schnittpunkte korrekt ist.

## Results
| # Lines    | # Intersections  | Duration (ms) |
|------------|------------------|---------------|
| 1001       | 11               | 19            |
| 10001      | 733              | 1640          |
| 100001     | 77138            | 156587        

### Extenal Library (geo)
The external library [geo](https://crates.io/crates/geo) is used to compare the results.
It yields the following results:
| # Lines    | # Intersections  | Duration (ms) |
|------------|------------------|---------------|
| 1001       | 11               | 105           |
| 10001      | 733              | 10425         |
| 100001     | 77138            | 1058787       |

## Justification
### Algorithm
- Simple but reliable Brute-force approach to compare all pairs of lines
- For each pair of lines, the intersection is calculated

### Tests
- Tests covering many edge cases
- Tests are comparing the results with the well-established library [geo](https://crates.io/crates/geo)

### Weaknesses
- The algorithm is not optimal and has a time complexity of O(n^2)
- Floating point arithmetic can lead to errors  