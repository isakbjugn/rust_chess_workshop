# Oppgave 5
> **Mål:** Implementere tårnets trekk

> **Hvor skal jeg jobbe:** [piece/rook.rs](piece/rook.rs)

I denne oppgaven skal vi implementere trekkene til tårnet. Du finner metodene som skal implementeres
`impl Piece for King {}`-blokken. Se etter metoden som inneholder en `todo!()`. I koden finner du også 
kommentarer som forklarer hva ulike metoder gjør.

Du finner også hint i [hint.md](hint.md).

## Tårnets trekk
Tårnet kan bevege seg så langt den vil mot nord, sør, øst og vest på brettet, helt til den når
enden av brettet eller en annen brikke. Tårnet kan bevege seg frem

1. *til og med* et felt som er tatt av en annen brikke  
2. til *men ikke med* et felt som er tatt av en brikke med samme farge

![Tårntrekk](../../images/moves/rook.gif)

> **PS! Rokade**  
> I likhet med kongen i oppgave 4 venter vi med å implementere rokade, som vanligvis også
omfatter tårnet (les mer om [rokade](https://no.wikipedia.org/wiki/Rokade)).

## Eksempel
Hvitt tårn i startposisjon på `a1`, med en svart bonde på `a5` og hvit løper på `c1` skal kunne
gå  skal kunne gå til `a2`, `a3`, `a4`, `a5`, `b1`:

```rust
let rook = Rook::new(Color::White, "a1".as_u8().unwrap());
let white_pieces = set!["c1"];
let black_pieces = set!["a5"];
let legal_moves = set!["a2", "a3", "a4", "a5", "b1"];
assert_eq_set!(legal_moves, rook.get_moves(&white_pieces, &black_pieces);
```

Se [hint.md](hint.md) for hint.