# Oppgave 3
> **Mål:** Implementere angrepstrekk for _hvit bonde_

> **Hvor skal jeg jobbe:** [piece/pawn.rs](piece/pawn.rs)

Denne oppgaven er en fortsettelse på forrige oppgave, nå skal vi implementere angrepstrekkene til den hvite bonden. 
Du må nå ta hensyn til hvor bonden står og hvor andre brikker står, men du trenger ikke å finne gyldige trekk for 
den sorte bonden.

- `get_capture_moves` trenger å vite hvor motstanderens brikker er. Utvid `get_moves` til å sende inn argument til `get_capture_moves` med fiendens posisjoner
- Utvid `get_capture_moves()`-metoden til å returnere trekk hvor bonden kan slå fiendtlige brikker, _dersom_ det står en brikke av motsatt farge der.

Du finner også hint i [hint.md](./hint.md).

## Bondens angrepstrekk
Bonden angriper ved å slå diagonalt fremover, dersom det står en brikke med motsatt farge plassert der. Se bonden 
til venstre i figuren under:

![Bondetrekk](../../images/moves/pawn.gif)

## Eksempel
Hvit bonde på `b4` skal kunne gå til `a5` eller `c5` dersom det står motstanderbrikker på `a5` og `c5`, og den kan 
generelt gå til `b5` hvis feltet er ledig (vanlig bevegelse fremover).

```rust
let pawn = Pawn::new(Color::White, "b4".as_u8().unwrap());
let opponent_piece_positions = set!["a5", "c5"];
let legal_moves = set!["a5", "b5", "c5"];
assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &opponent_piece_positions));
```

## Kjøring
```bash
cargo run 3
```
```bash
cargo test task_3
```

Se [hint.md](hint.md) for hint.