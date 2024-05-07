## Lab 02
Lesen Sie die SVG-Datei 'DeutschlandMitStaedten.svg' und ermitteln Sie die Flächen der einzelnen Bundesländer (bezüglich der in der Datei verwendeten Skala). Am Ende der Datei befinden sich Koordinaten von Städten, Versuchen Sie herauszufinden (bzw. lassen Sie das Ihren Rechner machen ;-), in welchem Bundesland diese jeweils liegen. 

## Lösung
| State                    | Area       | Cities               |
|--------------------------|------------|----------------------|
| Thüringen                | 13725      | Erfurt               |
| Schleswig-Holstein       | 13456      | Kiel                 |
| Sachsen-Anhalt           | 17451      | Magdeburg            |
| Sachsen                  | 15668      | Dresden              |
| Saarland                 | 2180       | Saarbrücken          |
| Rheinland-Pfalz          | 16914      | Mainz                |
| Nordrhein-Westfalen      | 28966      | Düsseldorf           |
| Niedersachsen            | 40634      | Hannover             |
| Mecklenburg-Vorpommern   | 19659      | Schwerin             |
| Hessen                   | 17978      | Wiesbaden            |
| Hamburg                  | 633        | Hamburg              |
| Bremen                   | 341        | Bremen               |
| Brandenburg              | 25276      | Potsdam              |
| Berlin                   | 766        | Berlin               |
| Bayern                   | 60026      | München              |
| Baden__x26__Württemberg  | 30522      | Stuttgart            |
### Begründung der Lösung

#### Limitierungen 
* Shoelace Formel funktioniert nur für simple Polygone.
* Ray Casting Algorithmus funktioniert nur für simple Polygone.
* Floating Point Arithmetic: Rundungsfehler können auftreten.
* Area implementiert als `äußere Fläche - innere Fläche`, auch wenn die innere Fläche nicht im Polygon liegt.
* Kategorisierung als Shell/Hole ist simplifiziert.
