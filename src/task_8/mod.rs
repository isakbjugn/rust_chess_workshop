mod board;
mod game;

/// # Oppgave 8
///
/// Nå er vi ferdige med å implementere trekkene til alle brikkene, og skal nå implementere
/// funksjonalitet for å sjekke om kongen er i sjakk. Vi skal i tillegg endre litt på `print()`
/// funksjonen i board, slik at kongen skrives ut i rødt dersom den står i sjakk (valgfritt).
///
/// At kongen står i sjakk betyr at den er under angrep, altså at en av motstanderen sine brikker
/// kan flytte til det feltet den kongen står på. Det er ikke lov å flytte din konge eller en annen
/// brikke slik at du setter din egen konge i sjakk.
///
/// Vi skal ikke enda implementere funksjonaliteten som hindrer deg i å sette deg selv i sjakk, kun
/// funksjonaliteten for å sjekke om en konge er i sjakk.
/// Gjør dette ved å implementere `is_check()` metoden i board.
/// Deretter kan du endre på `print()` for å printe ut kongen i rødt dersom den står i sjakk (valgfritt).
///
/// Testene i board vil teste at is_check() fungerer som forventet, men
/// det vil fortsatt være mulig å sette seg selv i sjakk ettersom metoden ikke er tatt i bruk i
/// get_legal_squares() enda. Dette skal vi vente med til oppgave 9.
///
/// Testene vil ikke verifisere at kongen printes ut i rødt, men dette kan du enkelt verifisere
/// selv med å kjøre programmet og deretter kopiere innholdet i games/checked_king.txt og lime inn
/// med ctrl+shift+V
///
///
/// Du finner også hint i [./hint.md](/hint.md).
pub fn main() {
    println!("Kjører game::main() i task_8");
    game::main()
}