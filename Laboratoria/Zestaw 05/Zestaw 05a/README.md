# Programowanie w Rustcie

## Zestaw 05a

### Zadanie 1

Napisz program z funkcją o nagłówku:

```
fn repeats(vector: ...) -> ...
```

która z danego wektora utworzy nowy z tymi samymi wartościami, ale tylko tymi, które się po sobie powtarzają. Na przykład:

```
repeats(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]) == vec![3, 3, 3, 3, 1, 1]
```

### Zadanie 2

Napisz program z funkcją o nagłówku:

```
fn unique_values(vector: ...) -> ...
```

która utworzy i zwróci nowy wektor, ale tylko z wartościami, które w danym wektorze się nie powtarzają (w zachowanej kolejności). Na przykład:

```
unique_values(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]) == vec![5, 6]
```

### Zadanie 3

Napisz program z funkcją o nagłówku:

```
fn count_repeats(vector1: ..., vector2: ...) -> ...
```

która zwróci liczbę elementów (z powtórzeniami) wektora vector1 zawartych w vector2:

```
count_repeats(&vec![1, 2, 1, 3], &vec![1, 2]) == 3
count_repeats(&vec![1, 2, 1, 3], &vec![12]) == 0
count_repeats(&vec![1, 2, 1, 3], &vec![1, 2, 2]) == 4
count_repeats(&vec![1, 2, 1, 3], &vec![1, 2, 2, 1]) == 6
```

### Zadanie 4

Napisz program z funkcją o nagłówku:

```
fn indexes(vector: ..., element: ...) -> ...
```

która zwróci wektor indeksów (licząc od zera), na których znajduje się w zadanym wektorze podany element.

### Zadanie 5

Napisz program wykorzystujący iteratory i tworzący wektor zawierający:

- małe litery alfabetu angielskiego,
- kwadraty 10 kolejnych liczb całkowitych,
- 10 kolejnych potęg dwójki,
- odwrotności wszystkich liczb od 1 do 20,
- liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4.

### Zadanie 6

Napisz program wykorzystujący iteratory i tworzący wektor zawierający:

- napisy krótsze niż 4 znaki,
- napisy nie zawierających liter 'a' ani 'A',
- napisy zawierających cyfry,
- te same napisy ale odwrócone,
- napisy zawierających podwojoną literę (na przykład: inny, pizza, brutto, lekki, dzienny, itp).
