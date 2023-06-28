# Oppgave 2

I denne oppgaven skal vi implementere de enkleste trekkene til bonden. 

I denne oppgaven skal vi implementere angrepstrekkene til bonden. I denne filen finner du en forklaring på hvordan 
bonden kan bevege seg, og en oppgavebeskrivelse. I koden vil det finnes kommentarer som beskriver hva ulike metoder 
gjør, og det står todo!() i metoden du skal implementere.

Du finner også hint i [hint.md](./hint.md).

## Bondens angrepstrekk
Bonden angriper ved å slå diagonalt fremover, dersom det står en brikke med motsatt farge plassert der. Se bonden 
til venstre i figuren under:

![Bondetrekk](../../images/moves/pawn.gif)

## Oppgavebeskrivelse
> Mål: Implementere angrepstrekk for bonden

> Hvor skal jeg jobbe: [piece/pawn.rs](piece/pawn.rs)

Denne oppgaven er en fortsettelse på forrige oppgave, nå skal vi implementere angrepstrekkene til
bonden.

Du må utvide `get_moves()` metoden til å støtte dette. Se etter en `todo!()`.

## Eksempel
Hvit bonde på b4 skal kunne gå til a5, b5 eller c5 dersom det står motstanderbrikker på a5 og c5.

```rust
let pawn = Pawn::new(Color::White, "b4".as_u8().unwrap());
let opponent_piece_positions = set!["a5", "c5"];
let legal_moves = set!["a5", "b5", "c5"];
assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &opponent_piece_positions));
```

Du finner også hint i [hint.md](./hint.md).