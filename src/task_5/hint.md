# Hint for oppgave 5

## Hint som er nyttige

<details>
<summary>Hint 1 – To fremgangsmåter</summary>

Som i flere oppgaver tidligere finnes det flere ulike måter å gå frem. Enten kan man
1. Iterere over felter i hver retning og stanse når man når en brikke (og håndtere ulikt avhengig av hvilken farge
brikken er), eller
2. Velge felter ukritisk, og deretter filtrere bort feltene som er ugyldige eller som tårnet ikke kan nå fordi det står
en brikke i veien.

</details>

<details>
<summary>Hint 2 – range i Rust</summary>

I `for`-løkker kan du har bruke for Rusts `range`:
 - `0..8` gir deg en iterator fra `0` til `7` (til men ikke med `8`)
 - `0..=8` gir deg en iterator med `0` til og med `8`
 - `(0..8).rev()` gir deg iteratoren med verdier i synkende rekkefølge

Disse kan du bruke på formen

```rust
for rank in (0..8) {
    // Gjør noe
}
```

</details>

<details>
<summary>Hint 3 – Vec</summary>

`Vec` kan være en egnet datastruktur for å representere en rekke med felter der rekkefølgen har betydning.

Les mer om `Vec` i [Rust-boka](https://doc.rust-lang.org/book/ch08-01-vectors.html) og i
[Rust-dokumentasjonen om Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

</details>

<details>
<summary>Hint 4 – Vektorer og iteratorer</summary>

Ofte er det nyttig å gjøre `HashSet` eller `Vec` om til iteratorer for å få tilgang til bestemte metoder, slik som
 - `map()` som avbilder fra hvert element i datastrukturen til noe annet
 - `filter()` som filtrerer hvert elemenet basert på en betingelse
 - `rev()` som reverserer retningen på iteratoren
 - `flat_map()` som produserer en iterator hvor hvert element, og deretter samler alle elementene i disse iteratorene
til én felles iterator (og kollapser slik en iterator i en iterator)
 - `cloned()` som tar deg fra iterator av `&T` til `T` (dette bruker vi for å lage f.eks. nye `Vec` av eksisterende `Vec`)
 - `collect()` som samler en iterator til en ny datastruktur (som f.eks. `Vec` eller `HashSet`)

For å gjøre en `Vec` eller `HashSet` om til en iterator kan du bruke `iter()` eller `into_iter()`. Mer om dette i eget hint.

Merk: Når du bruker `collect()` på en iterator kan Rust noen ganger utlede fra konteksten hvilken type du vil samle
iteratoren til, mens andre ganger er det nødvendig å oppgi type manuelt. Her har du to måter å gjøre det på:

Oppgi type når du tilordner:
```rust
let vec_1 = vec![1, 2, 3, 4, 5];
let vec_2: Vec<u8> = vec_1.iter().cloned().filter(|n| n > 2).collect();
assert_eq!(vec![3, 4, 5], vec_2)
```

Oppgi type ved samling (vec *collect*):
```rust
let vec_1 = vec![1, 2, 3, 4, 5];
let vec_2 = vec_1.iter().cloned().filter(|n| n > 2).collect::Vec<u8>();
assert_eq!(vec![3, 4, 5], vec_2)
```

</details>

<details>
<summary>Hint 5 – iter og into_iter</summary>

Her er noe som er lett å bli forvirret av selv etter å ha jobbet med Rust en stund: Hva er forskjellen på `iter()` og
`into_iter()`, disse to metodene som gir deg iteratorer fra vektorer eller andre datastrukturer?

`iter` er en iterator til det eksisterende objektet, og inneholder derfor referanser. Det vil si,

```rust
let vec_1 = vec![1, 2, 3, 4, 5]; // Denne er av typen Vec<u8>
vec_1.iter() // Denne iteratoren holder elementer av typen &u8
// Vi kan fremdeles lese fra vec_1
```

Dersom du vil lagre elementene fra `vec_1.iter()` i eksempelet over i en ny vektor, må du bruke `cloned()` for å få en
iterator som holder `u8`.

`into_iter()` konsumerer et eksisterende objekt, og gir deg en iterator av elementene i det tidligere objektet. Det vil si,

```rust
let vec_1 = vec![1, 2, 3, 4, 5]; // Denne er av typen Vec<u8>
vec_1.into_iter() // Denne iteratoren holder elementer av typen u8
// Vi kan ikke lese fra vec_1, for verdiene ble flyttet ved into_iter
```

`into_iter()` er det samme som `iter().cloned()`, men i stedet for å klone objektet, flytter vi heller alle verdiene inn
i iteratoren. Dersom vi ikke skal bruke `vec_1` fra eksempelet over videre, er det billigere å flytte enn å kopiere, og
med `into_iter()` kommuniserer vi dessuten at vi er ferdige med `vec_1`.

</details>

<details>
<summary>Hint 6 – filter_blocked_squares()</summary>

Dersom du velger fremgangsmåte to, og vil filtrere en bestemt retning (det vil si, en `Vec<(u8, u8)>` som representerer
alle feltene i en bestemt himmelretning, så finnes det en nyttemetode i `square.rs` som heter `filter_blocked_squares`.

Her er et eksempel på metoden i bruk:

La oss si at vi ser på et hvitt tårn med posisjon `A4` (`(0, 3)`), og det står en svart brikke på `A7`, og vi ser på
tårnets bevegelse i nordlig retning:

```rust
let move_direction = vec![(0, 4), (0, 5), (0, 6), (0, 7)];
let white_pieces = empty_set!();
let black_pieces = set!["a7"];
let legal_moves = set!["a5", "a6", "a7"];
assert_eq_set!(legal_moves, move_direction.filter_blocked_squares(&white_pieces, &black_pieces))
```

</details>

## Hint med løsningsforslag (fremgangsmåte 1)

<details>
<summary>Hint 7 – Utkast for algoritme for tårn-trekk</summary>

Hvis du kan bruke `range` til å finne de ulike retningene ut i fra tårnets posisjon, kan du bruke dette oppsettet for å
inkludere felter for hver retning:

```rust
let (x, y) = self.positions;
let mut moves = HashSet::new();

for file in correct_range { // Sett inn riktig retning her
    match (file, y) {
        s if team.contains(s) => break,
        s if rival_team.contains(s) => {
            moves.insert(s),
            break
        }
        s => moves.insert(s)
    }
}

// Gjenta for alle himmelretninger
```

</details>

<details>
<summary>Hint 8 – Ferdig algoritme for tårn-trekk</summary>

```rust
let (x, y) = self.positions;
let mut moves = HashSet::new();

for file in (0..x).rev() { // vestover
    match (file, y) {
        s if team.contains(s) => break,
        s if rival_team.contains(s) => {
            moves.insert(s),
            break
        }
        s => moves.insert(s)
    }
}

for file in (x..8) { // østover
    match (file, y) {
        s if team.contains(s) => break,
        s if rival_team.contains(s) => {
            moves.insert(s),
            break
        }
        s => moves.insert(s)
    }
}

for rank in (0..y).rev() { // sørover
    match (x, rank) {
        s if team.contains(s) => break,
        s if rival_team.contains(s) => {
            moves.insert(s),
            break
        }
        s => moves.insert(s)
    }
}

for rank in (x..8) { // nordover
    match (x, rank) {
        s if team.contains(s) => break,
        s if rival_team.contains(s) => {
            moves.insert(s),
            break
        }
        s => moves.insert(s)
    }
}

moves

```

</details>

## Hint med løsningsforslag (fremgangsmåte 2)

<details>
<summary>Hint 9 – Alternativ fremgangsmåte med vertikal og horisontal</summary>

Om du heller vil gå frem med å finne horisontalen og vertikalen tårnet står i gitt en posisjon `(x, y)`, kan du bruke
disse kodelinjene får å få to vektorer med posisjoner, som korresponderer til raden (vest-øst) og kolonnen (sør-nord)
som møtes i dette feltet:

```rust
let (x, y) = *position;
let vertical: Vec<(u8, u8)> = vec![(x, 0), (x, 1), (x, 2), (x, 3), (x, 4), (x, 5), (x, 6), (x, 7)];
let horizontal: Vec<(u8, u8)> = vec![(0, y), (1, y), (2, y), (3, y), (4, y), (5, y), (6, y), (7, y)];
```

</details>

<details>
<summary>Hint 10 – En mer elegant algoritme</summary>

I denne fremgangsmåten oppretter vi vektorer med hver himmelretning (med bruk av filtrering og reversering), og filterer
deretter denne med `filter_blocked_squares()`:

```rust
let (x, y) = *position;
let vertical: Vec<(u8, u8)> = vec![(x, 0), (x, 1), (x, 2), (x, 3), (x, 4), (x, 5), (x, 6), (x, 7)];
let horizontal: Vec<(u8, u8)> = vec![(0, y), (1, y), (2, y), (3, y), (4, y), (5, y), (6, y), (7, y)];

let north: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(_, new_y)| new_y > y).collect();
let south: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(_, new_y)| new_y < y).rev().collect();
let east: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(new_x, _)| new_x > x).collect();
let west: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(new_x, _)| new_x < x).rev().collect();

HashSet::<Vec<(u8, u8)>>::from_iter([north, south, east, west])
    .iter().flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
```

</details>