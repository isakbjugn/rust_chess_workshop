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
<summary>Hint 3 – Les om vektorer og iteratorer</summary>

Disse to delene av workshop-teorien kan være spesielt nyttig i denne oppgaven:

* [Vec](../../doc/teori/6-vektor-og-iterator.md#vec)
* [Iteratorer](../../doc/teori/6-vektor-og-iterator.md#iteratorer)

Du kan også lese mer om `Vec` i [Rust-boka](https://doc.rust-lang.org/book/ch08-01-vectors.html) og i
[Rust-dokumentasjonen om Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

</details>

<details>
<summary>Hint 4 – filter_blocked_squares()</summary>

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
<summary>Hint 5 – Utkast for algoritme for tårn-trekk</summary>

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
<summary>Hint 6 – Ferdig algoritme for tårn-trekk</summary>

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
<summary>Hint 7 – Alternativ fremgangsmåte med vertikal og horisontal</summary>

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
<summary>Hint 8 – En mer elegant algoritme</summary>

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