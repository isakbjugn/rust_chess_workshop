## Hint for oppgave 12

## Hint som er nyttige

<details>
<summary>Hint 1 – Hva innebærer det at kongen er sjakkmatt?</summary>

Fra oppgaveteksten: «Kongen er i sjakkmatt når han står i sjakk, og det ikke finnes noen trekk som kan forhindre at 
kongen fortsatt står i sjakk».

</details>

<details>
<summary>Hint 2 – Sjekk brikkene dine</summary>

Hva om du sjekker hver brikke for fargen `color` og ser om det finnes trekk som ikke fører til at kongen (fortsatt) 
står i sjakk?

Tips: Du har allerede skrevet en metode som finner disse trekkene ;)

</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 3 – get_legal_squares() og any() eller all()</summary>

```rust
fn is_checkmate(&self, color: Color) -> bool {
    self.get_positions(color).iter()
        .any(|position| !self.get_legal_squares(position).is_empty())
}
```
Merk at dette er det samme som:
```rust
fn is_checkmate(&self, color: Color) -> bool {
    self.get_positions(color).iter()
        .all(|position| self.get_legal_squares(position).is_empty())
}
```

på grunn av [De Morgans lover](https://en.wikipedia.org/wiki/De_Morgan%27s_laws).

</details>