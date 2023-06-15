## Hint for oppgave 10

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
<summary>Hint 3 – get_legal_squares() og any()</summary>

```rust
pub fn is_checkmate(&self, color: Color) -> bool {
        self.get_positions(color).iter()
            .any(|pos| !self.get_legal_squares(pos).is_empty())
    }
```

</details>