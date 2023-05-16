# Programowanie w Rustcie

## Zestaw 03a

### Zadanie 1

Napisz program z generatorem liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.

```
fn rand(seed: &mut ..., min_random: ..., max_random: ...) -> ...
```

### Zadanie 2

Napisz program z funkcją

```
swap_array(array: ..., i: usize, j: usize)
```

która zamieni wartości dwóch podanych elementów pewnej tablicy.

### Zadanie 3

Napisz program z funkcją

```
rand_perm(array: ..., seed: ...)
```

permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie.
