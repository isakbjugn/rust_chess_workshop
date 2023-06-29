# Oppgave 1
> **Mål:** Implementere åpningstrekk for _hvit bonde_

> **Hvor skal jeg jobbe:** [piece/pawn.rs](piece/pawn.rs)

I denne oppgaven skal vi implementere de aller enkleste trekkene til bonden, og vi begrenser oss til kun den _hvite_ 
bonden. I denne filen finner du en forklaring på hvordan bonden kan bevege seg, og en oppgavebeskrivelse. I koden 
vil det finnes kommentarer som beskriver hva ulike metoder gjør, og det står `todo!()` i metoden du skal implementere.

> **Obs! Les oppgaveteksten**  
> Ikke gap over for mye! Du skal ikke implementere alle bondetrekkene på en gang, men starte med det enkleste. Vi 
> skal implementere resten av trekkene i senere oppgaver.

Du finner også hint i [hint.md](./hint.md).

## Bondens trekk
Bonden er den mest grunnleggende brikken i sjakk, men dens bevegelsesmønster kan være litt forvirrende til å begynne 
med. Vi kommer til å fokusere på tre typer bondetrekk:
- Åpningstrekk: Bonden kan bevege seg ett eller to felt fremover
- Generell bevegelse: Bonden kan bevege seg ett felt fremover
- Angrepstrekk: Bonden kan slå brikker som befinner seg diagonalt foran bonden.

Bonden kan altså ikke gå til siden eller bakover, og den kan kun slå diagonalt. Se figuren under:

![Bondetrekk](../../images/moves/pawn.gif)

> **Merk!** I denne oppgaven skal vi kun implementere åpningstrekk for den hvite bonden.

## Oppgavebeskrivelse

I denne oppgaven jobber vi videre med `Pawn`, og skal implementere åpningstrekk for den hvite bonden. Du trenger 
altså ikke tenke på:
- svarte bønder,
- hvilke trekk bonden kan gjøre etter åpningstrekket,
- angrepstrekk,
- eller om andre brikker kan stå i veien (dette tar vi senere)

Du løser oppgaven ved å implementere metodene som står definert inni `impl Piece for Pawn {}`-blokken. (Se etter
`todo!()`) `Piece` er et slags *interface*, som kalles `trait` i Rust.

Vi skal lage to nyttemetoder, for å ha tilgang til private felt:
   - `get_color` (gir ut brikkens farge)
   - `get_position` (gir ut brikkens posisjon)
   - (Du finner også metoden `get_type` ferdigimplementert på `Piece`-traiten)

> **NB! Om posisjoner**  
> Det er verdt å merke seg at posisjoner i dette sjakkspillet er null-indekserte. Det vil si at feltet nede i
> venstre hjørne har verdien `(0, 0)` i koden, og `a1` i sjakk-domenet.

Samt to metoder vi trenger for å flytte bonden:
   - `move_piece` (endrer brikkens posisjon, foreløpig kun åpningstrekk og vanlig bevegelse fremover)
   - `get_moves` (henter ut gyldige felt en brikke kan flytte til)

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

Se [hint.md](hint.md) for hint.