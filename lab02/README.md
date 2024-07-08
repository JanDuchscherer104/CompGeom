## Lab 02
Lesen Sie die SVG-Datei 'DeutschlandMitStaedten.svg' und ermitteln Sie die Flächen der einzelnen Bundesländer (bezüglich der in der Datei verwendeten Skala). Am Ende der Datei befinden sich Koordinaten von Städten, Versuchen Sie herauszufinden (bzw. lassen Sie das Ihren Rechner machen ;-), in welchem Bundesland diese jeweils liegen. 

## Lösung
| State                    | Area       | ΔArea (%)  | Cities               |
|--------------------------|------------|------------|----------------------|
| Thüringen                | 13724.63   | 0.0000     | Erfurt               |
| Schleswig-Holstein       | 13456.44   | 0.5129     | Kiel                 |
| Sachsen-Anhalt           | 17450.55   | 0.6646     | Magdeburg            |
| Sachsen                  | 15667.91   | 0.2520     | Dresden              |
| Saarland                 | 2179.76    | 0.0686     | Saarbrücken          |
| Rheinland-Pfalz          | 16913.58   | 0.5463     | Mainz                |
| Nordrhein-Westfalen      | 28966.40   | 0.2433     | Düsseldorf           |
| Niedersachsen            | 40633.84   | 0.5415     | Hannover             |
| Mecklenburg-Vorpommern   | 19658.78   | -0.3751    | Schwerin             |
| Hessen                   | 17977.53   | 0.5063     | Wiesbaden            |
| Hamburg                  | 633.33     | -0.9935    | Hamburg              |
| Bremen                   | 340.93     | -4.1962    | Bremen               |
| Brandenburg              | 25275.92   | 0.6190     | Potsdam              |
| Berlin                   | 766.23     | 1.4862     | Berlin               |
| Bayern                   | 60026.12   | 0.4534     | München              |
| Baden-Württemberg        | 30522.31   | 0.7903     | Stuttgart            |

### Begründung der Lösung
#### Algorithmus
* Shoelace Formel zur Berechnung der Fläche eines Polygons
* Ray-Casting Algorithmus zur Bestimmung, ob ein Punkt innerhalb eines Polygons liegt

#### Tests
* Vielzahl an Tests zur Berechnung der Fläche eines Polygons und zur Bestimmung, ob ein Punkt innerhalb eines Polygons liegt

#### Vergleich mit tatsächlicher Fläche
* Vergleich mit tatsächlicher Bundeslandfläche (übernommen von https://www.statistikportal.de/de/bevoelkerung/flaeche-und-bevoelkerung).
* Leichte Abweichungen sind vorhanden.
* Separate Implementierung von Jan ergab das gleiche Ergebnis bzgl. Flächenunterschied -> Eventuell Abweichung des SVG oder eine der Limitierungen trifft zu.

#### Limitierungen
* Shoelace-Formel funktioniert nur für simple Polygone.
* Ray-Casting-Algorithmus funktioniert nur für simple Polygone.
* Floating Point Arithmetic: Rundungsfehler können auftreten.
* Area implementiert als äußere Fläche - innere Fläche, auch wenn die innere Fläche nicht im Polygon liegt.
* Kategorisierung als Shell/Hole ist simplifiziert.
