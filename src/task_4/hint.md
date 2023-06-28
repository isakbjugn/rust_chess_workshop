# Hint for oppgave 4

## Hint som er nyttige

<details>
<summary>Hint 1 – Les om match</summary>

Disse tre delene av workshop-teorien kan være nyttig i denne oppgaven:

* [match](../../doc/teori/4-match.md)
* [match og if](../../doc/teori/4-match.md#match-og-if)
* [Dobbel match](../../doc/teori/4-match.md#dobbel-match)

Spesielt dobbel `match` er fin å bruke dersom du både vil sjekke på brikkens farge og på posisjonen.

</details>

<details>
<summary>Hint 2 – Skille ut metoder</summary>

Blir det høy kompleksitet i `Pawn::get_moves()`? Du kan alltids lage nye metoder og kalle på disse fra `get_moves()`.
Merk at disse må legges i en `impl Pawn`-blokk (ikke `impl Piece for Pawn`) ettersom dette i så fall er metoder som 
ikke tilhører `Piece`-traiten.

Her er et eksempel:
```rust
impl Piece {
  fn get_forward_moves(all_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    todo!()
  }
  fn get_capture_moves(rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    todo!()
  }
}

impl Piece for Pawn {
  ...
  fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    let all_pieces = team.union(&rival_team).cloned().collect();
    let forward_moves = self.get_forward_moves(&all_pieces);
    let attack_moves = self.get_capture_moves(rival_team);
    forward_moves.union(&attack_moves).cloned().collect()
  }
}
```

</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 3 – Algoritme for å finne bondetrekk</summary>

```rust
impl Pawn {
    fn get_forward_moves(&self, other_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position;
        match (self.color, y) {
            (Color::White, 1) if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
            (Color::White, 1) => HashSet::from_iter([(x, 2), (x, 3)]),
            (Color::White, _) => HashSet::from_iter([(x, y + 1)]),
            (Color::Black, 6) if other_pieces.contains(&(x, y - 1)) => HashSet::new(),
            (Color::Black, 6) => HashSet::from_iter([(x, 5), (x, 4)]),
            (Color::Black, _) => HashSet::from_iter([(x, y - 1)])
        }.difference(other_pieces).cloned().collect()
    }

    fn get_capture_moves(&self, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        match self.color {
            Color::White => HashSet::from_iter([(x - 1, y + 1), (x + 1, y + 1)]),
            Color::Black => HashSet::from_iter([(x - 1, y - 1), (x + 1, y - 1)]),
        }.as_board_positions().intersection(rival_team).cloned().collect()
    }
}

impl Piece for Pawn {
  ...
  fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    let all_pieces: HashSet<_> = team.union(rival_team).cloned().collect();
    let moves = self.get_forward_moves(&all_pieces);
    let capture_moves = self.get_capture_moves(rival_team);
    moves.union(&capture_moves).cloned().collect()
  }
}
```

</details>