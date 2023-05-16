# Programowanie w Rustcie

## Zestaw 03b

### Zadanie 1

Napisz program z funkcją o nagłówku:

```
fn liczba_wystapien(napis: ..., znak: ...) -> ...
```

która zliczy i zwróci ile jest danych znaków w danym napisie.

### Zadanie 2

Napisz program z funkcją o nagłówku:

```
fn co_drugi_znak(napis: ...) -> ...
```

która zwróci napis zawierający co drugi znak z danego napisu.

### Zadanie 3

Napisz program z funkcją o nagłówku:

```
fn szyfruj(napis: ..., klucz: ...) -> ...
```

która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów. Przykłady:

```
szyfruj("Aladyn", 2) == "lAdany"
szyfruj("Aladyn", 3) == "alAnyd"
szyfruj("Aladyn", 4) == "dalAny"
szyfruj("Aladyn", 5) = "ydalAn"
szyfruj("koza", 3) == "zoka"
szyfruj("kaszanka", 3) == "saknazak"
szyfruj("kot Mruczek", 9) == "zcurM tokke"
szyfruj("kot Mruczek", 1) == "kot Mruczek"
szyfruj("kot Mruczek", 2) == "ok trMcuezk"
```

### Zadanie 4

Napisz program z funkcją o nagłówku ```wizytowka```, która otrzymuje w dwóch parametrach napisowych imię i nazwisko, a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska, przy czym w wyniku pierwsza litera imienia i nazwiska mają być duże, pozostałe małe. Na przykład, dla danych "jan" oraz "KOWALSKI" funkcja ma zwracać napis "J. Kowalski".

### Zadanie 5

Napisz program z funkcją o nagłówku:

```
fn rzymskie(napis: ...) -> ...
```

która dla napisu reprezentującego liczbę w zapisie rzymskim (zakładamy jego poprawność) zwraca liczbę reprezentowaną przez ów napis. Przykłady:

```
rzymskie("III") == 3
rzymskie("IX") == 9
rzymskie("XIX") == 19
rzymskie("MCMX") == 1910
```

### Zadanie 6

Napisz program z funkcją o nagłówku:

```
fn na_rzymskie(liczba: ...) -> ...
```

która dla danej liczby całkowitej zwraca jej zapis rzymski. Przykłady:

```
na_rzymskie(3) == "III"
na_rzymskie(9) == "IX"
na_rzymskie(19) == "XIX"
na_rzymskie(1910) == "MCMX"
```

### Zadanie 7

Napisz program z funkcją o nagłówku:

```
fn dodaj_pisemnie(a: ..., b: ...) -> ...
```

która doda dwie (zakładamy, że poprawne) liczby całkowite podane w argumentach jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis. Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże. Przykłady:

```
dodaj_pismnie("1", "3") == "4"
dodaj_pismnie("1", "3") == "4"
dodaj_pismnie("8", "3") == "11"
dodaj_pismnie("10", "23") == "33"
dodaj_pismnie("1", "0") == "1"
dodaj_pismnie("11", "00") == "11"
dodaj_pismnie("131", "9900") == "10031"
dodaj_pismnie("998", "7") == "1005"
dodaj_pismnie("24872947", "294729478") == "319602425"
dodaj_pismnie("5924729874298749827418582", "6782893629472094209740298") == "12707623503770844037158880"
```
