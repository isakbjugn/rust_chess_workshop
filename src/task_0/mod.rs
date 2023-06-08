mod board;
mod game;

/// Oppgave 0
/// I denne oppgaven skal vi opprette de grunnleggende datatypene som vi skal bruke i workshopen.
/// Det første vi skal lage er en representasjon av en sjakkbrikke, nærmere bestemt bonden.
/// Den burde ha felter som representerer:
///    - brikkens farge (som en enum-verdi)
///    - brikkens posisjon (som en tuppel av `(u8, u8)`)
///
/// Strukten burde ha følgende metoder:
///    - `new`, som instansierer en ny `Pawn` (en konstruktør)
///    - `print`, som returnerer brikkens tegn avhengig av farge ('♙' for svart og '♟' for hvit)
///
/// Når du har gjort det, kan du ut kommentere testen nedenfor for å se om det fungerer
///
/// Les i Rust-boka om:
///    - [Datatyper](https://doc.rust-lang.org/book/ch03-02-data-types.html)
///    - [Hvordan definere strukter](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
pub fn main() {
    println!("Køyrer game::main() i oppgåve 0");
    game::main()
}

/// Definer en `struct` som heter `Pawn`:
// todo!(Implementer Pawn)

/// Definer en `enum` som heter `Color`:
// todo!(Implementer Color)

#[cfg(test)]
mod tests {
    //use super::{Pawn, Color};

    #[test]
    fn pawn_struct_and_color_enum_exists() {
        /*
        let white_pawn = Pawn::new(Color::White, (0, 1));
        assert_eq!(white_pawn.print(), '♟');
        assert_eq!(white_pawn.position, (0, 1));
        let black_pawn = Pawn::new(Color::Black, (0, 6));
        assert_eq!(black_pawn.print(), '♙');
        assert_eq!(black_pawn.position, (0, 6));
         */
        assert!(false) // Denne gjør at testen kjører rødt. Fjern denne
    }
}