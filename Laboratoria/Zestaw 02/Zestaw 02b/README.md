# Programowanie w Rustcie

## Zestaw 02b

### Zadanie 1

Napisz program modyfikując wcześniejsze zadanie tak, by obliczenia były prowadzone nie dla sztywno zakodowanych funkcji i ich pochodnych, lecz by funkcję i pochodną można było przekazać do funkcji obliczającej miejsce zerowe (wskazówka: funkcja tablicuj na wykładzie).

### Zadanie 2

Napisz program z funkcją dwuargumentową, która zamieni wartości podanych w argumentach zmiennych (dla ustalenia uwagi: typu ```i32```).

### Zadanie 3

Napisz program z funkcją trójargumentową, która poprzestawia wartości swoich argumentów (dla ustalenia uwagi: typu ```i32```) tak, by były uporządkowane niemalejąco. W poniższych zadaniach staraj się dobierać najwygodniejsze konstrukcje powtarzania.

### Zadanie 4

Napisz program wyświetlający tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126).

### Zadanie 5

Napisz program z funkcją, która dla danego całkowitego dodatniego n zwraca numer iteracji, w której osiągamy jedynkę w problemie Collatza (np. dla ```n``` = ```12``` wynikiem jest ```9```).

### Zadanie 6

Napisz program z funkcją, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga.

### Zadanie 7

Napisz program z funkcją, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą.

### Zadanie 8

Napisz program z funkcją, która wyświetli rozkład podanej liczby na czynniki pierwsze.

### Zadanie 9

Napisz program z funkcją ```pow_mod(x: u128, n: u128, p: u128) -> u128``` która obliczy ```(x^n)%p``` taki sposób, by działało to prawidłowo dla jak największych danych.
- Wskazówka 1: skorzystaj z własności reszty z dzielenia dla iloczynu (czy też inaczej: iloczynu modulo).
- Wskazówka 2: w celu ewentualnej optymalizacji czasowej użyj algorytmu szybkiego potęgowania.
