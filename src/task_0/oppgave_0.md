# Oppgave 0
> Mål: Bli kjent med datatyper, `struct` og metoder

> Hvor skal jeg jobbe: [mod.rs](mod.rs)

I denne oppgaven skal vi opprette de grunnleggende datatypene som vi skal bruke i workshopen.
Det første vi skal lage er en representasjon av en sjakkbrikke, nærmere bestemt bonden.
Den burde ha felter som representerer:
- brikkens farge (som en enum-verdi)
- brikkens posisjon (som en tuppel av `(u8, u8)`)

Strukten burde ha følgende metoder:
- `new`, som instansierer en ny `Pawn` (en konstruktør)
- `print`, som returnerer brikkens tegn avhengig av farge ('♙' for svart og '♟' for hvit)

Når du har gjort det, kan du fjerne utkommenteringen for testen nederst i filen, for å se om det fungerer.

Les i Rust-boka om:
- [Datatyper](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Hvordan definere strukter](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

## NB! Om testing
I denne oppgaven er hovedinnholdet i testen utkommentert, og du må selv fjerne kommentar-tegnene når du er klar til 
å teste koden din.
> Dette skyldes at oppgaven går ut på å deklarere en `struct` og en `enum`, og Rust kan ikke kompilere dersom
> vi i testen refereres til noe som ikke finnes (da kan vi ikke kjøre testen i det hele tatt).
