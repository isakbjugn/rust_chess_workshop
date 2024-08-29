# Hint for oppgave 6

## Hint som er nyttige

<details>
<summary>Hint 1 – Har kongen noe til felles med springeren?</summary>

Kongen har egentlig mye til felles med springeren, rent algoritmisk, fordi den også kan flytte til bestemte felter
rundt seg gitt at disse ikke er utenfor brettet eller er opptatte av brikker med samme farge. Kan du gjenbruke koden
fra oppgave 5?

</details>

## Hint som avslører mulig løsning

<details>
<summary>Hint 2 – Gjenbruk springer-oppsettet</summary>

Du kan bruke akkurat samme tilnærming som for springeren, bare med et annet uttrykk for å finne de aktuelle feltene
rundt kongen:

```rust
let (x, y) = self.position.as_i8().unwrap();
let moves: HashSet<(i8, i8)> = HashSet::from([
    // Fyll inn de aktuelle posisjonene rundt kongen her
]);
moves.as_board_positions().difference(team).cloned().collect()
```

</details>

<details>
<summary>Hint 3 – Algoritme for kongetrekk</summary>

Her har vi fylt inn uttrykket for å velge felter rundt kongen, som vi deretter filtrerer avhengig av om de er på brettet
og om de ikke er opptatte av brikker av samme farge, som i hint 2:

```rust
let (x, y) = self.position.as_i8().unwrap();
let moves: HashSet<(i8, i8)> = HashSet::from([
    (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    (x - 1, y    ),             (x + 1, y    ),
    (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
]);
moves.as_board_positions().difference(team).cloned().collect()
```

PS! Kan du se hvorfor vi har plassert tuplene i dette mønsteret?

</details>