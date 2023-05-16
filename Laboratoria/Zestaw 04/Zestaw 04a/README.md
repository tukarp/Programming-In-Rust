# Programowanie w Rustcie

## Zestaw 04a

### Zadanie 1

Napisz program z funkcją o nagłówku:

```
fn dodaj_pisemnie(a: ..., b: ...) -> Result<...>
```

która doda dwie (nie zakładamy, że poprawne!) liczby całkowite podane w argumentach jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis (lub napisowy opis błędu). Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże. Użyj pomocniczej funkcji:

```
fn wartosc_cyfry(c: char) -> Result<...>
```

oraz operatora ```?```

### Zadanie 2

Napisz program z funkcją o nagłówku:

```
fn rzymskie(napis: ...) -> Result<...>
```

która dla napisu reprezentującego liczbę w zapisie rzymskim (nie zakładamy jego poprawności!) zwraca liczbę reprezentowaną przez ów napis lub napisowy opis błędu. Błędy mogą być dwojakie — niewłaściwa cyfra (o tym powiadomi nas poniższa funkcja pomocnicza) oraz niewłaściwa kolejność cyfr. Użyj pomocniczej funkcji:

```
fn wartosc_cyfry_rzymskiej(c: char) -> Result<...>
```

oraz operatora ```?```
