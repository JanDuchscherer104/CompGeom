In dem Tar-File 'strecken.tgz' (s.u.) befinden sich Dateien mit jeweils 4 Koordinaten pro Zeile. Diese stellen jeweils die x- und y-Koordinaten eines Start- bzw. Endpunkts einer Strecke dar. Lesen Sie jeweils eine Datei ein und ermitteln Sie die Anzahl der sich schneidenden (d.h. mindestens ein gemeinsamer Punkt) Strecken, indem Sie jedes Paar von Strecken gegeneinander testen. Messen Sie die pro Datei aufgewendete Zeit. Begründen Sie nachvollziehbar, warum die Anzahl der von Ihrem Programm jeweils gefundenen Schnittpunkte korrekt ist.

Calculate the number of intersecting lines (and not the number of intersections).

## Goals

- Read in a file with 4 coordinates per line into a vector of tuple<P1, P2>
- Test each pair of lines for intersection
- Measure time for each file

## Results

- File: `s_1000_1.dat`
  - Elements: 1001
  - Intersections: 725
  - Time: 6 ms
- File: `s_10000_1.dat`
  - Elements: 10001
  - Intersections: 9697
  - Time: 60 ms
- File: `s_100000_1.dat`
  - Elements: 100001
  - Intersections: 99691
  - Time: 580 ms

The complexity of the `count_intersecting_lines` is $\mathcal{O}(n^2)$, $\Omega(n)$.
The complexity `count_intersections` is $\mathcal{O}(n^2), \Omega(n^2)$ as no amortization is possible via an early exit.

**For Criterion**:

```rs
let css_line_pj = css_line(p, &input[j]);
let css_line_jp = css_line(&input[j], p);
if css_line_pj <= 0.0 && css_line_pj > css_line_jp
```

- File: `s_1000_1.dat`
  - Elements: 1001
  - Intersections: 127
  - Time: 20 ms
- File: `s_10000_1.dat`
  - Elements: 10001
  - Intersections: 4563
  - Time: 784 ms
- File: `s_100000_1.dat`
  - Elements: 100001
  - Intersections: 75575
  - Time: 35971 ms
