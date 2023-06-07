# Hint for oppgave 6

## Hint som er nyttige

<details>
<summary>Hint 1 – Har løperen nå til felles med tårnet?</summary>

I likhet med tårnet kan løperen bevege seg så langt den vil i fire retninger: nordøst, nordvest, sørøst, sørvest. Trolig
kan du gjenbruke mye av den omkringliggende koden du har skrevet for tårnet, om du endrer hvilke retninger du tar med i 
betraktning.

</details>

<details>
<summary>Hint 2 – Les om vektorer og iteratorer</summary>

I likhet med i oppgave 5 kan disse to delene av workshop-teorien kan være spesielt nyttig i denne oppgaven:

* [Vec](../../doc/teori/6-vektor-og-iterator.md#vec)
* [Iteratorer](../../doc/teori/6-vektor-og-iterator.md#iteratorer)

Du kan også lese mer om `Vec` i [Rust-boka](https://doc.rust-lang.org/book/ch08-01-vectors.html) og i
[Rust-dokumentasjonen om Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

</details>

<details>
<summary>Hint 3 – filter_blocked_squares()</summary>

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

## Hint som avslører mulig løsning

<details>
<summary>Hint 4 – Oppsett for å bruke ferdige diagonal-vektorer</summary>

I denne fremgangsmåten bruker vi de ferdige `get_south_east_diagonal()` og `get_north_east_diagonal()` for å opprette
vektorer med hver diagonal, som vi så må filtrere riktig. Deretter bruker vi `filter_blocked_squares()`:

```rust
let (x, y) = *position;
let se_diag = self.get_south_east_diagonal();
let ne_diag = self.get_north_east_diagonal();

let south_east: Vec<(u8, u8)> = // filtrer se_diag
let north_west: Vec<(u8, u8)> = // filtrer se_diag
let north_east: Vec<(u8, u8)> = // filtrer ne_diag
let south_west: Vec<(u8, u8)> = // filtrer ne_diag

HashSet::from_iter([south_east, north_west, north_east, south_west])
    .iter().flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
```

</details>

<details>
<summary>Hint 5 – Filtrering basert på ferdige diagonal-vektorer</summary>

Her filtrerer vi vektorene fra `get_south_east_diagonal()` og `get_north_east_diagonal()` med `filter()` og `rev()` 
der det trenges, og filtrerer til slutt med `filter_blocked_squares()`:

```rust
let (x, y) = *position;
let se_diag = self.get_south_east_diagonal();
let ne_diag = self.get_north_east_diagonal();

let south_east: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_x, new_y)| new_x > x && new_y < y).collect();
let north_west: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_x, new_y)| new_x < x && new_y > y).rev().collect();
let north_east: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_x, new_y)| new_x > x && new_y > y).collect();
let south_west: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_x, new_y)| new_x < x && new_y < y).rev().collect();

HashSet::from_iter([south_east, north_west, north_east, south_west])
    .iter().flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
```

</details>