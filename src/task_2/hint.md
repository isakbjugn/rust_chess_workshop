# Hint for oppgave 1

## Hint som er nyttige

<details>
<summary>Hint 1 – Hvordan bruke HashSet</summary>

En matematisk mengde, som i Rust er implementert som `HashSet`, kjennetegnes ved at elementenes rekkefølge ikke har
betydning, og at ingen elementer opptrer flere ganger. I metodene som beregner lovlige trekk for en brikke er det ofte
nyttig å bruke metoder knyttet til `HashSet`, som for eksempel:

* `HashSet::union`: Gir alle verdiene som finnes i to `HashSet`
* `HashSet::intersection`: Gir alle verdiene som er felles for to `HashSet`
* `HashSet::difference`: Gir alle verdiene som er unike for ett `HashSet` sammenliknet med et annet
* `HashSet::symmetric_difference`: Gir alle verdiene som er unike for to `HashSet` og *ikke* finnes i begge

Les mer om `HashSet` og lær hvordan disse metodene brukes i [Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
</details>

<details>
<summary>Hint 2 – Noen nyttemetoder for u8 og i8</summary>

Det kan være at du i løpet av implementasjonen din har behov for enten å konvertere (u8, u8) til (i8, i8) (dette er
ikke nødvendig for å løse oppgaven, men kan være at det er aktuelt for din implementasjon). I så fall kan du dra nytte
av  ferdiglagde metoder som finnes i `src/square.rs`:

* `(u8, u8).as_i8()`: Konverterer `(u8, u8)` til `(i8, i8)`
* `(i8, i8).as_u8()`: Konverterer `(i8, i8)` til `(u8, u8)`, og feiler dersom den opprinnelige tuppelen inneholder negative
  tall
* `HashSet<(u8, u8)>.as_board_positions`: Filtrerer bort tuppelverdier som ikke finnes på brettet, f.eks. `(0, 10)`
* `HashSet<(i8, i8)>.as_board_positions`: Filtrerer bort tuppelverdier som ikke finnes på brettet, f.eks. `(0, 10)`, i
  tillegg til å konvertere fra `(i8, i8)` til `(u8, u8)`

Ta gjerne en ekstra titt i `src/square.rs` i fall du kan finne noe som bli nyttig senere!
</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 3 – Algoritme for å finne bondetrekk</summary>

Én mulig fremgangsmåte for å beregne bondetrekk er å undersøke hvor bonden befinner seg for øyeblikket, og så inkludere
felter avhengig av dette. En alternativ fremgangsmåte er å inkludere felter litt mer ukritisk, og deretter filtrere bort
det som ikke er gyldige posisjoner.

Slik kan du gå frem med den ukritiske tilnærmingen:

```rust
let (x, y) = self.position;
match self.color {
    Color::White => HashSet::from_iter([(x - 1, y + 1), (x + 1, y + 1)]),
    Color::Black => HashSet::from_iter([(x - 1, y - 1), (x + 1, y - 1)]),
}.as_board_positions().intersection(rival_team).cloned().collect()
```

</details>