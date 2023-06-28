# Hint for oppgave 11

## Hint som er nyttige
<details>
<summary>
Hint 1
</summary>

Fra oppgaveteksten: "Gyldige trekk er kun de som resulterer i en stilling der kongen din ikke er i sjakk", altså for 
hvert trekk i `get_legal_moves()` slik den ser ut nå, så er trekket ugyldig dersom det resulterer i et brett der du 
står i sjakk.

Kan du sjekke for hvert mulige trekk for en brikke om står i sjakk?

</details>

<details>

<summary>
Hint 2 - Mulig fremgangsmåte
</summary>

- for hvert trekk i `get_legal_moves()`
  - konstruer ett nytt brett ved å klone det forrige `Board { pieces: self.pieces.clone() }`
  - utfør trekket
  - sjekk om kongen står i sjakk i denne posisjonen med `is_check()`
  - fjern trekket dersom kongen står i sjakk

Du kan gjøre trekkene om til en iterator og deretter bruke `.filter()`

</details>

## Hint som avslører mulig løsning

<details>
<summary>
Hint  - En mulig løsning
</summary>

```rust
pub fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
    let color = self.get_square_color(position).expect("Inga brikke på vald posisjon");
    let team = self.get_positions(color);
    let rival_team = self.get_positions(color.opposite());
    let piece = self.pieces.get(position).expect("Inga brikke på vald posisjon.");
    let moves = piece.get_moves(&team, &rival_team);
    moves
        .into_iter()
        .filter(|&square| {
            let mut new_board = Board {
                pieces: self.pieces.clone()
            };
            new_board.move_piece(&piece.get_position(), square);
            !new_board.is_check(color)
        }).collect()
}
```
</details>