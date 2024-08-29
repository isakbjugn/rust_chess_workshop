# Hint for oppgave 3

## Hint som er nyttige

<details>
<summary>Hint 1 – Les mer om HashSet</summary>

Ta en titt på [HashSet](../../doc/teori/7-hashset-og-hashmap.md) i workshop-teorien. Spesielt operasjonen
`HashSet::intersection()` kan være nyttig for denne oppgaven.

Du kan også lese mer om `HashSet` og denne metoden i
[Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html).

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
<summary>Hint 3 – Algoritme for å finne angrepstrekk for bonden</summary>

Én mulig fremgangsmåte for å beregne bondens angrepstrekk er å undersøke hvor bonden befinner seg for øyeblikket, og så inkludere
felter avhengig av dette. En alternativ fremgangsmåte er å inkludere felter litt mer ukritisk, og deretter filtrere bort
det som ikke er gyldige posisjoner.

Slik kan du gå frem med denne litt ukritiske tilnærmingen:

```rust
let (x, y) = self.position.as_i8().unwrap();
HashSet::from([(x - 1, y + 1), (x + 1, y + 1)])
    .as_board_positions()
    .intersection(rival_team).cloned().collect()
```

</details>