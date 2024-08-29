# Oppgave 2
> **Mål:** Ta hensyn til om andre brikker står i veien for _hvit bonde_

> **Hvor skal jeg jobbe:** [piece/pawn.rs](piece/pawn.rs)

> **Hint:** [hint.md](./hint.md)

I denne oppgaven fortsetter vi på forrige oppgave, og skal utvide `get_forward_moves()` til å ta hensyn til at en annen brikke kan stå i veien for bondens steg fremover. Du kan fortsatt se bort i fra:
- svarte bønder
- trekk for å angripe

I denne filen finner du en oppgavebeskrivelse, og en forklaring på hvordan bonden kan bevege seg. I koden
vil det finnes kommentarer som beskriver hva ulike metoder gjør, og det står `todo!()` i metoden du skal implementere.

## Oppgavebeskrivelse

- `get_forward_moves` trenger å vite hvor alle brikkene på brettet står, for å finne ut om et felt er ledig for bondens bevegelse. Dette kan du sende inn som argument i `get_moves`. Husk å oppdatere signaturen til `get_forward_moves`!
- Utvid `get_forward_moves()` til å returnere gyldige til å returnere gyldige forovertrekk for bonden uansett hvor bonden befinner seg, også om det er brikker i veien. Du kan se bort i fra nederste rad (der den hvite bonden aldri befinner seg).

Oppgaven er fullført når testene kjører grønt.

> **Merk!** I denne oppgaven skal vi kun implementere vanlig bevegelse for den hvite bonden.

## Bondens trekk
Bonden er den mest grunnleggende brikken i sjakk, men dens bevegelsesmønster kan være litt forvirrende til å begynne
med. Vi kommer til å fokusere på tre typer bondetrekk:
- Forovertrekk, som omfatter:
    - Åpningstrekk: Bonden kan bevege seg ett eller to felt fremover
    - Generell bevegelse: Bonden kan bevege seg ett felt fremover
- Angrepstrekk: Bonden kan slå brikker som befinner seg diagonalt foran bonden.

> **Obs!:**  
> Hvis en brikke står rett foran bonden, er både kort og lang åpning hindret.

Bonden kan altså ikke gå til siden eller bakover, og den kan kun slå diagonalt. Se figuren under:

![Bondetrekk](../../images/moves/pawn.gif)

## Kjøring
```bash
cargo run 2
```
```bash
cargo test task_2
```

## Les mer om:
   - [Metoder som muterer](https://doc.rust-lang.org/book/ch05-03-method-syntax.html?#defining-methods)
   - [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
   - [Referanser](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
