## Hint for oppgave 13

## Hint som er nyttige
<details>
<summary>Hint 1 – Hvordan vet vi om kongen eller tårnet har flyttet seg?</summary>

Den enkleste måten å holde styr på om kongen eller tårnet ikke har flyttet på seg, er å legge til et nytt felt i `struct King` og i `struct Rook`. Dette feltet kan vi kalle `has_moved`.

I så fall må vi sette en verdi for `has_moved` i konstruktøren for strukten. Vi må også oppdatere `has_moved` inni `move_piece`.

Verdien av `has_moved` kan du gjøre tilgjengelig utenfor struken med en metode som f.eks. `can_castle()`.

</details>

<details>
<summary>Hint 2 – Downcaste fra Piece til King</summary>

Inni `get_castle_moves()` har du behov for å kalle på `can_castle() for både `King` og `Rook`. Dette gjør du ved å _downcaste_ fra `trait Piece` til den aktuelle strukten, slik:

```rust
let piece = self.pieces.get(&position).expect("Inga brikke på posisjonen");
let king = piece.downcast_ref::<King>().expect("Brikken var ikkje ein konge");
if king.can_castle()
    ...
```
</details>

<details>
<summary>Hint 3 – Implementer reglene for rokade én etter én</summary>

Inni `get_castle_moves()` må du ta hensyn til alle reglene for rokade. Dette kan du gjøre én etter én med if-setninger og match-setninger.

Du kan for eksempel starte med å initialisere et tomt `HashSet`, og først ta for deg det ene tårnet, og undersøke om alle betingelsene for rokade er oppfylt. Hvis ja, så setter du inn rokadetrekket inn i hashsettet. Hvis en av betingelsene feiler, går du videre til å undersøke neste tårn.
</details>

<details>
<summary>Hint 4 – Gjenbruk kode fra is_check</summary>

I `is_check` har vi allerede implementert funksjonaliteten for å avgjøre om et felt er truet av en bestemt farge. Dette kan du trekke ut i en egen metode (`is_square_threatened`), og ta i bruk for å avgjøre om feltene kongen skal bevege seg fordi ikke er truet.
</details>

## Hint som avslører en mulig løsning
<details>
<summary>Hint 5 – Oppdatering av struct King</summary>

```rust
#[derive(Clone)]
pub struct King {
    pub color: Color,
    pub position: (u8, u8),
    pub has_moved: bool,
}

impl King {
    pub fn can_castle(&self) -> bool {
        !self.has_moved
    }
}

impl Piece for King {
    fn new(color: Color, position: (u8, u8)) -> Self {
        King {
            color,
            position,
            has_moved: false,
        }
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.has_moved = true;
        self.position = target;
    }
}
```

</details>

<details>
<summary>Hint 6 – Mulig implementasjon av get_castle_moved</summary>

```rust
fn get_castle_moves(&self, king_position: &(u8, u8), ) -> HashSet<(u8, u8)> {
    let king = self.pieces.get(king_position).expect("Inga brikke på vald posisjon")
        .downcast_ref::<King>().expect("Brikka er ikkje ein konge");
    if !king.can_castle() || self.is_check(king.color)  { return HashSet::new(); }

    let mut castle_moves = HashSet::new();
    let (_, y) = king.position;

    for rook_position in [(0, y), (7, y)] {
        match self.pieces.get(&rook_position) {
            Some(piece) => {
                match piece.downcast_ref::<Rook>() {
                    Some(rook) if rook.can_castle() => {
                        if rook_position.0 < king.position.0 {
                            // Dronningfløyen

                            // Regel: Inga brikker i vegen
                            for square_between in [(1, y), (2, y), (3, y)] {
                                if self.pieces.get(&square_between).is_some() {
                                    break
                                }
                            }
                            // Regel: Kongen kan ikkje gå på eit felt truga av motstandaren
                            for king_path_square in [(1, y), (2, y)] {
                                if self.is_square_threatened(&king_path_square, king.color) {
                                    break
                                }
                            }
                            castle_moves.insert((2, y));
                        } else {
                            // Kongefløyen

                            // Regel: Inga brikker i vegen
                            for square_between in [(5, y), (6, y)] {
                                if self.pieces.get(&square_between).is_some() {
                                    break
                                }
                            }
                            // Regel: Kongen kan ikkje gå på eit felt truga av motstandaren
                            for king_path_square in [(5, y), (6, y)] {
                                if self.is_square_threatened(&king_path_square, king.color) {
                                    break
                                }
                            }
                            castle_moves.insert((6, y));
                        }
                    },
                    _ => break
                }
            },
            None => break
        }
    }
    castle_moves
}
```

</details>

