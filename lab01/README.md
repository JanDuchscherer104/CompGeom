## Excercise 01
In dem Tar-File 'strecken.tgz' (s.u.) befinden sich Dateien mit jeweils 4 Koordinaten pro Zeile. Diese stellen jeweils die x- und y-Koordinaten eines Start- bzw. Endpunkts einer Strecke dar. Lesen Sie jeweils eine Datei ein und ermitteln Sie die Anzahl der sich schneidenden (d.h. mindestens ein gemeinsamer Punkt) Strecken, indem Sie jedes Paar von Strecken gegeneinander testen. Messen Sie die pro Datei aufgewendete Zeit. Begründen Sie nachvollziehbar, warum die Anzahl der von Ihrem Programm jeweils gefundenen Schnittpunkte korrekt ist.

## Usage
```bash
cargo run src/main.rs
```

## Results
| # Lines    | # Intersections  | CPU Time (ms) |
-------------------------------------------------
| 1001       | 11               | 16            |
| 10001      | 733              | 1670          |
| 100001     | 77138            | 168664        |

### Extenal Library (geo)
The external library [geo](https://crates.io/crates/geo) is used to compare the results.
It yields the following results:
| # Lines    | # Intersections  | CPU Time (ms) |
-------------------------------------------------
| 1001       | 11               | 94            |
| 10001      | 733              | 9253          |
| 100001     | 77138            | 925482        |

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

### Benchmarking
- Interestingly, my implementation is faster than the external library, even though I'm using the same brute-force approach to compare all pairs of lines and the calculation of the intersection seems identical
- Measured CPU time instead of wall time to avoid inaccuracies due to other processes, but still, the results might be inaccurate as the experiments were only run once and conducted on a single machine with other processes running in the background
