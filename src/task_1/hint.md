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
<summary>Hint 2 – match</summary>

Når du har posisjonen til bonden og vil returnere verdier avhengig av hva posisjonen er, er `match` et godt valg. Du
kan for eksempel matche på farge:

```rust
match self.color {
    Color::White => {} // Gjør logikk for hvit bonde
    Color::Black => {} // Gjør logikk for sort bonde
}
```

</details>

<details>
<summary>Hint 3 – match og if</summary>

Du kan også kombinere en generell `match` med en mer spesifikk `if` for å spisse betingelsene:

```rust
match self.color {
    Color::White if self.position == some_value => {} // Spesiallogikk for hvit bonde
    Color::White => {} // Generell logikk for hvit bonde
    Color::Black if self.position == some_other_value => {} // Spesiallogikk for sort bonde
    Color::Black => {} // Generell logikk for sort bonde
}
```

</details>

<details>
<summary>Hint 4 – Dobbel match</summary>

Det vi så i hint 3 kan også kombineres i en dobbel `match`. Det går ut på å sette variabler sammen i en tuppel
(`(var_1, var_2)`) og så matche på den:

```rust
match (self.color, self.position.1) {
    (Color::White, some_value) => {} // Spesiallogikk for hvit bonde
    (Color::White, _) => {} // Generell logikk for hvit bonde
    (Color::Black, some_other_value) => {} // Spesiallogikk for sort bonde
    (Color::Black, _) => {} // Generell logikk for sort bonde
}
```

Vi bruker understrek `_` i `match`-armer (*arm* i dette tilfellet er det vi ofte kaller *greiner* i `if`-setninger)
for å vise at vi *ikke bryr oss om* hva verdien er, og at vi ikke skal bruke den videre  i armen.

</details>

<details>
<summary>Hint 5 – Union på HashSet</summary>

En matematisk mengde, som i Rust er implementert som `HashSet`, kjennetegnes ved at elementenes rekkefølge ikke har
betydning, og at ingen elementer opptrer flere ganger. I metodene som beregner lovlige trekk for en brikke er det ofte
nyttig å bruke metoder knyttet til `HashSet`, blant annet:

* `HashSet::union`: Gir alle verdiene som finnes i to `HashSet`

Brukes på følgende måte
```rust
let set_1 = HashSet::from_iter([1, 2, 3]);
let set_2 = HashSet::from_iter([3, 4, 5]);
let set_union: HashSet<_> = set_1.union(&set_2).collect()
assert_eq(set_union, HashSet::from_iter([1, 2, 3, 4, 5]))
```

* `HashSet::difference`: Gir alle verdiene som er unike for ett `HashSet` sammenliknet med et annet

Brukes på følgende måte
```rust
let set_1 = HashSet::from_iter([1, 2, 3]);
let set_2 = HashSet::from_iter([3, 4, 5]);
let set_difference: HashSet<_> = set_1.difference(&set_2).collect()
assert_eq(set_union, HashSet::from_iter([1, 2]))
```

Les mer om `HashSet` og lær hvordan disse metodene brukes i [Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 6 – En algoritme for bondetrekk</summary>

Bonden blir hindret fra å gå fremover hvis det står en annen brikke direkte foran, uansett hvordan farge den brikken
har. Vi kan bruke dette og `match` til å lage en enkel algoritme for å finne gyldige trekk.

Her følger en enkel kode for å finne bondens vanlige trekk (for å gå fremover, ikke for å slå andre brikker):

```rust
let (x, y) = self.position;
let other_pieces: HashSet<_> = team.union(rival_team).collect();
match (self.color, y) {
    (Color::White, _) if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
    (Color::White, 1) if !other_pieces.contains(&(x, y + 2)) => HashSet::from_iter([(x, 2), (x, 3)]),
    (Color::White, _) => HashSet::from_iter([(x, y + 1)]),
    (Color::Black, _) if other_pieces.contains(&(x, y - 1)) => HashSet::new(),
    (Color::Black, 6) if !other_pieces.contains(&(x, y - 2)) => HashSet::from_iter([(x, 5), (x, 4)]),
    (Color::Black, _) => HashSet::from_iter([(x, y - 1)])
}
```

</details>

<details>
<summary>Hint 6 – En litt mer konsis algoritme for bondetrekk</summary>

Dersom vi bruker litt flere `HashSet`-metoder kan vi gjøre `match`-logikken litt enklere, og så gjøre filtrering
etterpå. Her er en alternativ løsning:

```rust
let (x, y) = self.position;
let other_pieces: HashSet<_> = team.union(rival_team).cloned().collect();
match (self.color, y) {
    (Color::White, 1) if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
    (Color::White, 1) => HashSet::from_iter([(x, 2), (x, 3)]),
    (Color::White, _) => HashSet::from_iter([(x, y + 1)]),
    (Color::Black, 6) if other_pieces.contains(&(x, y - 1)) => HashSet::new(),
    (Color::Black, 6) => HashSet::from_iter([(x, 5), (x, 4)]),
    (Color::Black, _) => HashSet::from_iter([(x, y - 1)])
}.difference(&other_pieces).cloned().collect()
```

</details>