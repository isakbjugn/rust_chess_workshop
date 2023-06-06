# Hint for oppgave 6

## Hint som er nyttige

<details>
<summary>Hint 1 – Har løperen nå til felles med tårnet?</summary>

I likhet med tårnet kan løperen bevege seg så langt den vil i fire retninger: nordøst, nordvest, sørøst, sørvest. Trolig
kan du gjenbruke mye av den omkringliggende koden du har skrevet for tårnet, om du endrer hvilke retninger du tar med i 
betraktning.

</details>

<details>
<summary>Hint 2 – To fremgangsmåter</summary>

Som med tårnet kan du
1. Iterere over felter i hver retning og stanse når man når en brikke (og håndtere ulikt avhengig av hvilken farge
   brikken er), eller
2. Velge felter ukritisk, og deretter filtrere bort feltene som er ugyldige eller som løperen ikke kan nå fordi det står
   en brikke i veien.

</details>

<details>
<summary>Hint 3 – Vec</summary>

`Vec` kan være en egnet datastruktur for å representere en rekke med felter der rekkefølgen har betydning.

Les mer om `Vec` i [Rust-boka](https://doc.rust-lang.org/book/ch08-01-vectors.html) og i
[Rust-dokumentasjonen om Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

</details>

<details>
<summary>Hint 4 – filter_blocked_squares()</summary>

Dersom du velger fremgangsmåte to, og vil filtrere en bestemt retning (det vil si, en `Vec<(u8, u8)>` som representerer
alle feltene i en bestemt himmelretning, så finnes det en nyttemetode i `square.rs` som heter `filter_blocked_squares`.

Her er et eksempel på metoden i bruk:

La oss si at vi ser på en hvit løper posisjon `A4` (`(0, 3)`), og det står en svart brikke på `D7`, og vi ser på
løperens bevegelse i nordøstlig retning:

```rust
let move_direction = vec![(1, 4), (2, 5), (3, 6), (4, 7)];
let white_pieces = empty_set!();
let black_pieces = set!["d7"];
let legal_moves = set!["b5", "c6", "d7"];
assert_eq_set!(legal_moves, move_direction.filter_blocked_squares(&white_pieces, &black_pieces))
```

</details>

<details>
<summary>Hint 5 – Egne metoder for Bishop</summary>

Om du ønsker å skille ut egne metoder til `Bishop`, som du kan kalle på fra en metode inni `impl Piece for Pawn {}`, kan
du legge dette inni en egen `impl`-blokk kun for `Bishop`

```rust
impl Bishop {
    my_custom_method(&self) -> MyResultType {
        // Implementasjon
}
```

</details>

## Hint som avslører mulig løsning

<details>
<summary>Hint 6 – Utkast for algoritme for løper-trekk</summary>

Hvis du kan bruke `range` til å finne de ulike retningene ut i fra løperens posisjon, kan du bruke dette oppsettet for å
inkludere felter for hver retning. Obs! Dette krever mer tenking enn i tilfellet med tårnet, fordi du skal iterere langs
diagonalen, hvor både `x` og `y` endrer seg.

```rust
let (x, y) = self.positions;
let mut moves = HashSet::new();

for index in insert_range { // Sett inn riktig retning her
    match insert_tuple { // Sett inn riktig tuppel her
        s if team.contains(s) => break,
        s if rival_team.contains(s) => {
            moves.insert(s),
            break
        }
        s => moves.insert(s)
    }
}

// Gjenta for alle diagonaler
```

</details>

<details>
<summary>Hint 7 – Oppsett for en alternativ fremgangsmåte</summary>

I denne fremgangsmåten oppretter vi vektorer med hver diagonal (med bruk av filtrering og reversering), og filterer
deretter denne med `filter_blocked_squares()`:

```rust
let (x, y) = *position;
let se_diag = // beregne diagonal som går nedover i sørøstlig retning
let ne_diag = // beregne diagonal som går oppover i nordøstlig retning

let south_east: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_x, new_y)| new_x > x && new_y < y).collect();
let north_west: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_x, new_y)| new_x < x && new_y > y).rev().collect();
let north_east: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_x, new_y)| new_x > x && new_y > y).collect();
let south_west: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_x, new_y)| new_x < x && new_y < y).rev().collect();

HashSet::from_iter([south_east, north_west, north_east, south_west])
    .iter().flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
```

</details>

<details>
<summary>Hint 8 – Beregne diagonaler</summary>

Denne er veldig gøy å finne ut av! Bruk den om du står fast, men kudos til deg om du klarer det på egenhånd!

```rust
pub fn get_south_east_diagonal(position: &(u8, u8)) -> Vec<(u8, u8)> {
    let sum = position.0 + position.1;
    match sum {
        0 => vec![(0, 0)],
        1 => vec![(0, 1), (1, 0)],
        2 => vec![(0, 2), (1, 1), (2, 0)],
        3 => vec![(0, 3), (1, 2), (2, 1), (3, 0)],
        4 => vec![(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)],
        5 => vec![(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)],
        6 => vec![(0, 6), (1, 5), (2, 4), (3, 3), (4, 2), (5, 1), (6, 0)],
        7 => vec![(0, 7), (1, 6), (2, 5), (3, 4), (4, 3), (5, 2), (6, 1), (7, 0)],
        8 => vec![(1, 7), (2, 6), (3, 5), (4, 4), (5, 3), (6, 2), (7, 1)],
        9 => vec![(2, 7), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2)],
        10 => vec![(3, 7), (4, 6), (5, 5), (6, 4), (7, 3)],
        11 => vec![(4, 7), (5, 6), (6, 5), (7, 4)],
        12 => vec![(5, 7), (6, 6), (7, 5)],
        13 => vec![(6, 7), (7, 6)],
        14 => vec![(7, 7)],
        _ => panic!()
    }
}

pub fn get_north_east_diagonal(position: &(u8, u8)) -> Vec<(u8, u8)> {
    let difference = position.1 as i8 - position.0 as i8;
    match difference {
        7 => vec![(0, 7)],
        6 => vec![(0, 6), (1, 7)],
        5 => vec![(0, 5), (1, 6), (2, 7)],
        4 => vec![(0, 4), (1, 5), (2, 6), (3, 7)],
        3 => vec![(0, 3), (1, 4), (2, 5), (3, 6), (4, 7)],
        2 => vec![(0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7)],
        1 => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)],
        0 => vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
        -1 => vec![(1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5), (7, 6)],
        -2 => vec![(2, 0), (3, 1), (4, 2), (5, 3), (6, 4), (7, 5)],
        -3 => vec![(3, 0), (4, 1), (5, 2), (6, 3), (7, 4)],
        -4 => vec![(4, 0), (5, 1), (6, 2), (7, 3)],
        -5 => vec![(5, 0), (6, 1), (7, 2)],
        -6 => vec![(6, 0), (7, 1)],
        -7 => vec![(7, 0)],
        _ => panic!()
    }
}
```

</details>
