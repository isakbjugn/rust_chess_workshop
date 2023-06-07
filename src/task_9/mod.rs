mod board;
mod game;

/// # Oppgave 9
///
/// Nå skal vi ta i bruk is_check() metoden fra forrige oppgave, slik at vi forhindrer muligheten
/// for å sette seg selv i sjakk. Det innebærer også at dersom du står i sjakk, så er de eneste gyldige
/// trekkene de som enten flytter kongen til et felt som ikke er under angrep, eller blokkerer veien
/// til brikka som truer kongen. Sagt på en enklere måte så er gyldige trekk kun de som resulterer i en
/// posisjon der du ikke er i sjakk.
///
/// Dette kan vi implementere med å utvide metoden `get_legal_squares()` i board.rs, ved å bl.a ta i
/// bruk is_check().
///
/// Du finner også hint i [./hint.md](/hint.md).
pub fn main() {
    println!("Kjører game::main() i task_9");
    game::main()
}