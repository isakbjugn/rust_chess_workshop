# Oppgave 8
> **Mål:** Implementere løperens trekk

> **Hvor skal jeg jobbe:** [piece/bishop.rs](piece/bishop.rs)

> **Hint:** [hint.md](./hint.md)

I denne oppgaven skal vi implementere trekkene til løperen. Du finner metodene som skal implementeres
`impl Piece for Bishop {}`-blokken. Se etter metoden som inneholder en `todo!()`. I koden finner du også kommentarer 
som forklarer hva ulike metoder gjør.

## Løperens trekk
Løperen kan bevege seg langs alle diagonaler: Nordøst, nordvest, sørøst, sørvest, helt
til den når enden av brettet eller en annen brikke. Løperen kan bevege seg

1. *til og med* et felt som er tatt av en brikke med motsatt farge
2. til *men ikke med* et felt som er tatt av en brikke med samme farge

![Løpertrekk](../../images/moves/bishop.gif)

## Eksempel
Hvit løper i startposisjon på `c1`, med en hvit bonde på `b2` og ingen brikker som blokkerer i
nordløstlig retning. Løperen skal kunne gå til `d2`, `e3`, `f4`, `g5`, `h6`:

```rust
let bishop = Bishop::new(Color::White, "c1".as_u8().unwrap());
let white_pieces = set!["b2"];
let black_pieces = empty_set!();
let legal_moves = set!["d2", "e3", "f4", "g5", "h6"];
assert_eq_set!(legal_moves, bishop.get_moves(&white_pieces, &black_pieces));
```

## Kjøring
```bash
cargo run 8
```
```bash
cargo test task_8
```
