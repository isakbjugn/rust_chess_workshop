# Hint for oppgave 8

## Hint som er nyttige
<details>
<summary>
Hint 1 - Hjelpefunksjon
</summary>

Det kan være nyttig å lage en funksjon for å hente ut posisjonen til kongen av en bestemt farge

```rust
fn get_king_position(&self, color: Color) -> &(u8, u8)
```
</details>

<details>
<summary>
Hint 2 - Fremgangsmåte
</summary>

- hent posisjonen til kongen av gitt farge
- hent alle motstanderens brikker
- gå gjennom alle motstanderens brikker og sammenligne brikkens `get_moves()` med kongens posisjon

</details>

<details>
<summary>
Hint 3 - Hjelpefunksjon for å printe kongen i rødt
</summary>

Det kan være nyttig å lage en funksjon som returnerer posisjonen til en kongen som står i sjakk _dersom_ det finnes 
en konge som står i sjakk. Denne funksjonen burde returnere en `Option<&(u8, u8)>`.

Forslag: Hvis du lager denne funksjonen kan du gjerne ta den i bruk i `Board::print()` for å vise at kongen står i 
sjakk, f.eks. med å markere den i rødt.

Husk at det ikke er mulig at begge kongene står i sjakk samtidig.

</details>

## Hint som avslører mulig løsning

<details>
<summary>
Hint 3 - En mulig løsning
</summary>

```rust
fn is_check(&self, color: Color) -> bool {
    let king_position = self.get_king_position(color);
    let team = self.get_positions(color);
    let rival_team = self.get_positions(color.opposite());

    for piece in self.get_pieces_iter(color.opposite()) {
        if piece.get_moves(&rival_team, &team).contains(king_position) {
            return true;
        }
    }
    false
}

fn get_king_position(&self, color: Color) -> &(u8, u8) {
    self.pieces.values().find(|piece| {
        piece.get_color() == color && piece.get_name() == KING_NAME
    }).unwrap().get_position()
}

fn get_pieces_iter(&self, color: Color) -> impl Iterator<Item=&Box<dyn Piece>> {
    self.pieces.values().filter(move |piece| piece.get_color() == color)
}

fn get_checked_king(&self) -> Option<&(u8, u8)> {
    for color in [Color::White, Color::Black] {
        if self.is_check(color) {
            return Some(self.get_king_position(color))
        }
    }
    None
}

fn print(&self, legal_squares: Option<&HashSet<(u8, u8)>>) {
    let board = self.create_board();
    let empty_hashset = HashSet::new();
    let legal_squares = legal_squares.unwrap_or(&empty_hashset);
    let checked_king = self.get_checked_king();

    println!("   {:_<33}", "");
    for (y, row) in board.iter().rev().enumerate() {
        print!("{}  ", 8 - y);
        for (x, piece) in row.iter().enumerate() {
            match *piece {
                '_' if legal_squares.contains(&(x as u8, 7 - y as u8)) => print!("| {} ", "□".green()),
                '_' => print!("|   "),
                c if checked_king == Some(&(x as u8, 7 - y as u8)) => print!("| {} ", c.to_string().red()),
                c if legal_squares.contains(&(x as u8, 7 - y as u8)) => print!("| {} ", c.to_string().red()),
                c => print!("| {} ", c)
            }
        }
        println!("|")
    }
    println!("   {:͞<33}", ""); // \u{035E}
    println!("     A   B   C   D   E   F   G   H");
}
```

</details>
