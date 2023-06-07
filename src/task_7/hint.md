# Hint for oppgave 7

## Hint som er nyttige

<details>
<summary>Hint 1 – Har dronninga noe til felles med tårnet og løperen?</summary>

Trekkene til dronninga er kombinasjonen av trekkene til tårnet og løperen. Du kan prøve å gjenbruke kode fra dem.

</details>

<details>
<summary>Hint 2 - Gjenbruke kode</summary>

For å kunne gjenbruke koden for å finne trekkene til tårenet og løperen i implementasjonen av dronninga må vi skrive litt om. Slik det er nå så ligger f.eks trekkene til tårnet i `Rook` sin `get_moves()` metode. Du vet at det er en metode og ikke en vanlig funksjon ettersom den tar inn `&self`, og vi må da altså ha en instans av en Rook for å kunne kalle den, eks. `my_rook.get_moves()`. Vi ønsker derimot å kunne gjøre noe ala `Rook::get_rook_moves()`. Da må vi lage en `get_rook_moves()` funksjon i Rook som __ikke__ har `&self` som parameter, og heller sende inn posisjonen som parameter.

</details>

## Hint som avslører mulig løsning

<details>
<summary>Hint 3 – Delvis implementasjon</summary>

I Rook og i Bishop:
```rust
impl Rook {
    pub fn get_rook_moves(position: &(u8, u8)) -> HashSet<Vec<(u8, u8)>> {
        // Flytte implementasjonen fra get_moves til hit
    }
}


impl Piece for Rook {
    /* resten av impl Piece */
    
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        Rook::get_rook_moves(&self.position).iter()
            .flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
    }
}
```

I Queen:
```rust
fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    let mut move_directions = Rook::get_rook_moves(&self.position);
    move_directions.extend(Bishop::get_bishop_moves(&self.position));
    move_directions.iter()
        .flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
}
```
</details>
