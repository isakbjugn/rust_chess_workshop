# Hint for oppgave 7

## Hint som er nyttige

<details>
<summary>Hint 1 – Har dronninga noe til felles med tårnet og løperen?</summary>

Trekkene til dronninga er kombinasjonen av trekkene til tårnet og løperen. Du kan prøve å gjenbruke kode fra dem.

</details>

<details>
<summary>Hint 2 – Gjenbruke kode</summary>

For å kunne gjenbruke koden for å finne trekkene til tårenet og løperen i implementasjonen av dronninga må vi skrive 
litt om. Slik det er nå så ligger f.eks trekkene til tårnet i `Rook` sin `get_moves()`-metode. Dette er en metode, 
ikke en funksjon, ettersom den tar inn `&self`, og er kun tilgjengelig for å kalle fra en instans av en `Rook`. Vi 
ønsker derimot å kalle på noe à la `Rook::get_rook_moves()`. Da må vi lage en `get_rook_moves()`-funksjon i `Rook` 
som _ikke_ er en metode, men som er en offentlig, assosiert funksjon.

> En funksjon definert innenfor en `impl`-blokk er
>  * en metode dersom den tar `&self` (eller `&mut self`) som argument, og kan kun kalles fra en instans, som med  
   `self.get_moves()`
>  * en assosiert funksjon dersom den ikke tar inn `&self` (eller `&mut self`), og vi kan kalle den uten instans 
   dersom den er offentlig (markert med `pub` nøkkelordet), som med `Rook::get_rook_moves()`
> 
> Les mer om `struct`, metoder og assosierte funksjoner i [struct og trait](../../doc/teori/5-struct-og-trait.md) i 
> workshop-teorien.

</details>


<details>
<summary>Hint 3 – Egne metoder for Rook/Bishop</summary>

Om du ønsker å skille ut egne metoder til `Bishop`, som du kan kalle på fra en metode inni `impl Piece for Pawn {}`, kan
du legge dette inni en egen `impl`-blokk kun for `Bishop`

```rust
impl Bishop {
    fn my_custom_method(&self) -> MyResultType {
        // Implementasjon
}
```
For å kunne kalle denne utenfor et `Bishop`-objekt må vi definere den uten `&self` i funksjonssignaturen:

```rust
impl Bishop {
    pub fn my_custom_method() -> MyResultType {
        // Implementasjon
}
```

Dersom vi f.eks. vil bruke posisjonen `(u8, u8)` i en slik funksjon, må vi sende den inn som argument.

Les mer om `struct` og `impl`-blokker i [Implementere struct](../../doc/teori/5-struct-og-trait.md) i workshop-teorien.

</details>

## Hint som avslører mulig løsning

<details>
<summary>Hint 4 – Delvis implementasjon</summary>

I `Rook` og i `Bishop`:
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

I `Queen`:
```rust
fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    let mut move_directions = Rook::get_rook_moves(&self.position);
    move_directions.extend(Bishop::get_bishop_moves(&self.position));
    move_directions.iter()
        .flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
}
```
</details>
