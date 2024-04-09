In dem Tar-File 'strecken.tgz' (s.u.) befinden sich Dateien mit jeweils 4 Koordinaten pro Zeile. Diese stellen jeweils die x- und y-Koordinaten eines Start- bzw. Endpunkts einer Strecke dar. Lesen Sie jeweils eine Datei ein und ermitteln Sie die Anzahl der sich schneidenden (d.h. mindestens ein gemeinsamer Punkt) Strecken, indem Sie jedes Paar von Strecken gegeneinander testen. Messen Sie die pro Datei aufgewendete Zeit. Begründen Sie nachvollziehbar, warum die Anzahl der von Ihrem Programm jeweils gefundenen Schnittpunkte korrekt ist.

## Goals

- Read in a file with 4 coordinates per line into a vector of tuple<P1, P2>
- Test each pair of lines for intersection
- Measure time for each file

### Extra Goals

- Calculate the intersection points

Number of Intersections: 2276; Time elapsed in count_intersections is: 12.549542ms for file_name 's_1000_1.dat'
Number of Intersections: 180580; Time elapsed in count_intersections is: 753.254833ms for file_name 's_10000_1.dat'
Number of Intersections: 17675533; Time elapsed in count_intersections is: 73.622455834s for file_name 's_100000_1.dat'
