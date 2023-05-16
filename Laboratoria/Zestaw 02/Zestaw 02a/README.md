# Programowanie w Rustcie

## Zestaw 02a

### Zadanie 1

Napisz program definiujący dwie funkcje

```
fn f(x: f64) -> f64
```

```
fn fp(x: f64) -> f64
```

(spełniające odpowiednie założenia; druga jest pochodną pierwszej) napisz funkcję

```
fn met_newt(x0: f64, eps: f64, N: u128) -> f64
```

realizującą znajdowanie przybliżonego miejsca zerowego metodą Newtona — w trzech wersjach:
- iteracyjnej z pętlą ```loop``` (z ewentualnymi ```break``` ```continue``` ```return```),
- iteracyjnej z pętlą ```while``` (bez żadnych ```break``` ```continue``` ```return```),
- rekurencyjnej,
- iteracyjnej z pętlą ```for``` (z ewentualnymi ```break``` ```continue``` ```return```).
