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
<summary>Hint 2 – Oppsett for springer-algoritme</summary>

Dersom vi velger den ukritiske tilnærmingen hvor vi velger alle aktuelle felter rundt springeren og deretter filterer
bort felter utenfor brettet eller de som er opptatte av en brikke med samme farge (som finnes i `team` `HashSet`-et,
kan du gå frem slik:

```rust
let (x, y) = self.position.as_i8().unwrap();
let moves: HashSet<(i8, i8)> = HashSet::from_iter([
    // Fyll inn de aktuelle posisjonene rundt springeren her
]);
moves.as_board_positions().difference(team).cloned().collect()
```

</details>

<details>
<summary>Hint 3 – Algoritme for springertrekk</summary>

Her har vi fylt inn de aktuelle feltene rundt springeren, og bruker oppsettet fra hint 2 for å filtrere bort felter
utenfor brettet og som er opptatt av brikker med samme farge:

```rust
let (x, y) = self.position.as_i8().unwrap();
let moves: HashSet<(i8, i8)> = HashSet::from_iter([
                   (x - 1, y + 2), (x + 1, y + 2),
   (x - 2, y + 1),                                 (x + 2, y + 1),
   
   (x - 2, y - 1),                                 (x + 2, y - 1),
                   (x - 1, y - 2), (x + 1, y - 2),
]);
moves.as_board_positions().difference(team).cloned().collect()
```

PS! Kan du se hvorfor vi har plassert tuplene i dette mønsteret?

</details>