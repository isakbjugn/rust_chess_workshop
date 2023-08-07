# Oppgave 10
> **Mål:** Kunne finne ut om kongen er i sjakk

> **Hvor skal jeg jobbe:** [board.rs](board.rs)

Nå er vi ferdige med å implementere trekkene til alle brikkene, og skal nå implementere funksjonalitet for å sjekke 
om kongen er i sjakk. Vi skal i tillegg endre litt på `print()`-funksjonen i board, slik at kongen skrives ut i rødt 
dersom den står i sjakk (valgfritt). Se etter metoden som inneholder en `todo!()`. I koden finner du også kommentarer
som forklarer hva ulike metoder gjør.

## Hva vil det si at kongen er i sjakk?
At kongen står i sjakk betyr at den er under angrep, altså at en av motstanderen sine brikker kan flytte til det 
feltet den kongen står på. Det er ikke lov å flytte din konge eller en annen brikke slik at du setter din egen konge 
i sjakk.

Vi skal ikke enda implementere funksjonaliteten som hindrer deg i å sette deg selv i sjakk, kun funksjonaliteten for 
å sjekke om en konge er i sjakk.

Gjør dette ved å implementere `is_check()` metoden i board.

Deretter kan du endre på `print()` for å printe ut kongen i rødt dersom den står i sjakk (valgfritt).

## Kjøring
```bash
cargo run 10
```
```bash
cargo test task_10
```

## Testing
Testene i `Board` vil teste at `is_check()` fungerer som forventet, men det vil fortsatt være mulig å sette seg selv 
i sjakk ettersom metoden ikke er tatt i bruk i `get_legal_squares()` enda.

Testene vil ikke verifisere at kongen printes ut i rødt, men dette kan du enkelt verifisere
selv med å kjøre programmet og deretter kopiere innholdet i [games/checked_king.txt](../../games/checked_king.txt) og lime inn
med `ctrl+shift+V`.

Du finner også hint i [hint.md](hint.md).