# Hint for oppgave 1

## Hint som er nyttige

<details>
<summary>Hint 1 – Når er bonden i åpningsposisjon?</summary>

Ettersom bonden kun kan gå fremover og aldri bakover, så kan du være sikker på at

* Hvit bonde er i åpningsposisjon når `y = 1`
* Sort bonde er i åpningsposisjon når `y = 6`

Kan du bruke dette for å avgjøre når bonden skal kunne gå to felt fremover, versus når den kun får gå ett felt?

</details>

<details>
<summary>Hint 2 – Les om match</summary>

Disse tre delene av workshop-teorien kan være spesielt nyttig i denne oppgaven:

* [match](../../doc/teori/4-match.md)
* [match og if](../../doc/teori/4-match.md#match-og-if)
* [Dobbel match](../../doc/teori/4-match.md#dobbel-match)

</details>

<details>
<summary>Hint 3 – Les mer om HashSet</summary>

Ta en titt på [HashSet](../../doc/teori/7-hashset-og-hashmap.md) i workshop-teorien. Spesielt operasjonene 
`HashSet::union()` og `HashSet::difference()` kan være nyttige for denne oppgaven.

Du kan også lese mer om `HashSet` og disse metodene i
[Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html).

</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 4 – En algoritme for bondetrekk</summary>

Bonden blir hindret fra å gå fremover hvis det står en annen brikke direkte foran, uansett hvordan farge den brikken
har. Vi kan bruke dette og `match` til å lage en enkel algoritme for å finne gyldige trekk.

Her følger en enkel kode for å finne bondens vanlige trekk (for å gå fremover, ikke for å slå andre brikker):

```rust
let (x, y) = self .position;
let other_pieces: HashSet<_ > = team.union (rival_team).collect();
match ( self .color, y) {
(Color::White, _) if other_pieces.contains( & (x, y + 1)) => HashSet::new(),
(Color::White, 1) if ! other_pieces.contains( & (x, y + 2)) => HashSet::from_iter([(x, 2), (x, 3)]),
(Color::White, _) => HashSet::from_iter([(x, y + 1)]),
(Color::Black, _) if other_pieces.contains( & (x, y - 1)) => HashSet::new(),
(Color::Black, 6) if !other_pieces.contains( & (x, y - 2)) => HashSet::from_iter([(x, 5), (x, 4)]),
(Color::Black, _) => HashSet::from_iter([(x, y - 1)])
}
```

</details>

<details>
<summary>Hint 5 – En litt mer konsis algoritme for bondetrekk</summary>

Dersom vi bruker litt flere `HashSet`-metoder kan vi gjøre `match`-logikken litt enklere, og så gjøre filtrering
etterpå. Her er en alternativ løsning:

```rust
let (x, y) = self .position;
let other_pieces: HashSet<_ > = team.union (rival_team).cloned().collect();
match ( self .color, y) {
(Color::White, 1) if other_pieces.contains( & (x, y + 1)) => HashSet::new(),
(Color::White, 1) => HashSet::from_iter([(x, 2), (x, 3)]),
(Color::White, _) => HashSet::from_iter([(x, y + 1)]),
(Color::Black, 6) if other_pieces.contains(& (x, y - 1)) => HashSet::new(),
(Color::Black, 6) => HashSet::from_iter([(x, 5), (x, 4)]),
(Color::Black, _) => HashSet::from_iter([(x, y - 1)])
}.difference( & other_pieces).cloned().collect()
```

</details>