## Hint for oppgave 13

## Hint som er nyttige
<details>
<summary>Hint 1 – Finn ut hvilket tårn som inngår i rokaden</summary>

Når du vet hvilken side av kongen rokaden skal utføres på (dronningfløyen til venstre eller kongefløyen til høyre), kan du også avgjøre hvor tårnet som er med i rokaden står.

Deretter kan du kalle på `self.move_piece` først for kongen, deretter for tårnet.
</details>

## Hint som avslører en mulig løsning
<details>
<summary>Hint 2 – En mulig implementasjon</summary>

```rust
fn castle(&mut self, king_position: &(u8, u8), target_square: (u8, u8)) {
    match target_square {
        (2, y) if y == king_position.1 => {
            self.move_piece(king_position, target_square);
            self.move_piece(&(0, y), (3, y))
        },
        (6, y) if y == king_position.1 => {
            self.move_piece(king_position, target_square);
            self.move_piece(&(7, y), (5, y))
        },
        _ => panic!("Ugyldig rokadetrekk")
    }
}
```

</details>
