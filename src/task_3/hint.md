# Hint for oppgave 3

## Hint som er nyttige

<details>
<summary>Hint 1 – Hvor står springeren?</summary>

I likhet med i oppgave 2 finnes det to ulike tilnærminger til å finne springerens trekk. Vi kan
1. Se hvor springeren står og velge felter ut i fra posisjonen, eller
2. Velge felter rundt springeren ukritisk og deretter filtere bort posisjoner som
    1. er utenfor brettet, eller
    2. er opptatt av en brikke av samme farge

</details>

## Hint som avslører mulig løsning

<details>
<summary>Hint 1 – Algoritme for springertrekk</summary>

Dersom vi velger den ukritiske tilnærmingen hvor vi velger alle aktuelle felter rundt springeren og deretter filterer
bort felter utenfor brettet eller de som er opptatte av en brikke med samme farge, kan du gå frem slik:

```rust
let (x, y) = self.position.as_i8().unwrap();
let moves: HashSet<(i8, i8)> = HashSet::from_iter([
    (x - 2, y + 1), (x - 2, y - 1), // to kolonner til venstre
    (x - 1, y + 2), (x - 1, y - 2), // én kolonne til venstre
    (x + 1, y + 2), (x + 1, y - 2), // én kolonne til høyre
    (x + 2, y + 1), (x + 2, y - 1), // to kolonner til høyre
]);
moves.as_board_positions().difference(team).cloned().collect()
```

</details>