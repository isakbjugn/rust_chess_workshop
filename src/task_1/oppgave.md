# Oppgave 1
> **Mål:** Implementere forovertrekk for _hvit bonde_

> **Hvor skal jeg jobbe:** [piece/pawn.rs](piece/pawn.rs)

> **Hint:** [hint.md](./hint.md)

I denne oppgaven skal vi implementere de første trekkene til bonden, og vi begrenser oss til kun den _hvite_ bonden. I denne filen finner en oppgavebeskrivelse, og en reglene for hvordan bonden kan bevege seg. I koden vil det finnes kommentarer som beskriver hva ulike metoder gjør, og det står `todo!()` i metodene du skal implementere.

> **Obs! Les oppgaveteksten**  
> Ikke gap over for mye! Du skal ikke implementere alle bondetrekkene på en gang, men starte med trekkene bonden kan gjøre forover, uten å tenke på om det står brikker i veien. Vi skal implementere resten av trekkene i senere oppgaver.

## Oppgavebeskrivelse

Vi skal lage to nyttemetoder, for å ha tilgang til private felt:
- `get_color` (gir ut brikkens farge)
- `get_position` (gir ut brikkens posisjon)
- (Du finner også metoden `get_type` ferdigimplementert på `Piece`-traiten)

Vi skal også implementere to metoder vi trenger for å flytte bonden:
- `move_piece` (endrer brikkens posisjon, foreløpig kun åpningstrekk og vanlig bevegelse fremover)
- `get_forward_moves` (gir oss gyldige forovertrekk for bonden)

> **Kun åpningstrekk!**  
> Vi skal kun implementere forovertrekk for hvit bonde, og du kan derfor se bort fra:
> - om det står andre brikker i veien,
> - om bonden kan slå andre brikker,
> - eller hvordan svarte brikker kan bevege seg.
>
> Se avsnittet under for forklaring på hvordan bonden kan bevege seg.

> **NB! Om posisjoner**  
> Det er verdt å merke seg at posisjoner i dette sjakkspillet er null-indekserte. Det vil si at feltet nede i
> venstre hjørne har verdien `(0, 0)` i koden, og `a1` i sjakk-domenet.

## Bondens trekk
Bonden er den mest grunnleggende brikken i sjakk, men dens bevegelsesmønster kan være litt forvirrende til å begynne 
med. Vi kommer til å fokusere på tre typer bondetrekk:
- Forovertrekk, som omfatter:
  - Åpningstrekk: Bonden kan bevege seg ett eller to felt fremover
  - Generell bevegelse: Bonden kan bevege seg ett felt fremover
- Angrepstrekk: Bonden kan slå brikker som befinner seg diagonalt foran bonden.

Bonden kan altså ikke gå til siden eller bakover, og den kan kun slå diagonalt. Se figuren under:

![Bondetrekk](../../images/moves/pawn.gif)

## Eksempel
Som vist i figuren over: Hvit bonde på `d2` skal kunne gå til `d3` og `d4` i åpningstrekket:

```rust
let pawn = Pawn::new(Color::White, "d2".as_u8().unwrap());
let legal_moves = set!["d3", "d4"];
assert_eq_set!(legal_moves, pawn.get_moves(&empty_set!(), &empty_set!()));
```

## Testing
Oppgaven er fullført når testene kjører grønt.
Det kan være nyttig å først kjøre `cargo run 1` for å få printet ut et sjakkbrett (foreløpig kun med bønder) for 
lettere å kunne visualisere posisjoner.

## Kjøring
```bash
cargo run 1
```
```bash
cargo test task_1::
```

## Les mer om:
   - [Metoder som muterer](https://doc.rust-lang.org/book/ch05-03-method-syntax.html?#defining-methods)
   - [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
   - [Referanser](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
