# Oppgave 0
> Mål: Bli kjent med datatyper, `struct` og metoder

> Hvor skal jeg jobbe: [mod.rs](mod.rs)

I denne oppgaven skal vi opprette grunnleggende datatyper som vi kommer til å trenge i workshopen:

1. En `enum` som heter `Color` som representerer fargene til brikkene
2. En `struct` kalt `Pawn` som representerer en sjakkbrikke (bonden)

## Oppgavebeskrivelse

### Color
Enumen `Color` skal ha to mulige verdier: `White` og `Black`. En enum defineres slik:

```rust
pub enum Color {
    // Variantene til Color
}
```
Se eksempler på bruk av [enum](../../doc/teori/4-match.md) i teoridelen.

### Pawn
Strukten `Pawn` skal ha to felter:
- brikkens farge (som en enum-verdi, bruk `Color` til dette)
- brikkens posisjon (som en tuppel av `(u8, u8)`)

Du definerer en `struct` slik:

```rust
pub struct Pawn {
    // Feltene til bonden
}
```

Se flere eksempler om [struct](../../doc/teori/5-struct-og-trait.md) i teoridelen.

`Pawn` skal også ha to metoder:

- `new`, som instansierer en ny `Pawn` (en konstruktør)
- `print`, som returnerer brikkens tegn avhengig av farge ('♙' for svart og '♟' for hvit)

For å legge til metoder til en `struct` må du lage en `impl`-blokk:
```rust
impl Pawn {
    // Metoder til Pawn
}
```

## Testing

Når du har implementert `Color` og `Pawn` som beskrevet over, kan du fjerne utkommenteringen for testen nederst i filen, for å se om det fungerer.

> ### NB! Om testing
> I denne oppgaven er hovedinnholdet i testen utkommentert, og du må selv fjerne kommentar-tegnene når du er klar til
å teste koden din.
> 
> Dette skyldes at oppgaven går ut på å deklarere en `struct` og en `enum`, og Rust kan ikke kompilere dersom
> vi i testen refereres til noe som ikke finnes (da kan vi ikke kjøre testen i det hele tatt).

## Kjøring
```bash
cargo run 0
```
```bash
cargo test task_0
```

Les i Rust-boka om:
- [Datatyper](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Hvordan definere strukter](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)