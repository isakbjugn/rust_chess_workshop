# Hint for oppgave 1

## Hint som er nyttige

<details>
<summary>Hint 1 – Anta at bonden er i åpningsposisjon</summary>

Hvit bonde er i åpningsposisjon når `y = 1`. Hva er da `y`-verdien til de to gyldige åpningstrekkene?

</details>

<details>
<summary>Hint 2 – HashSet</summary>

`Pawn::get_moves()` returnerer et `HashSet`. Intensjonen bak `HashSet` er en mengde som kan holde verdier av samme 
type, hvor rekkefølge ikke har betydning.

Du kan opprette et nytt `HashSet` slik, og sette inn verdier:
```rust
let empty_hash_set = HashSet::new()
empty_hash_set.insert((0, 0))
```

eller opprette et `HashSet` direkte fra verdier:
```rust
let filled_hash_set = HashSet::from_iter([(0, 0), (0, 1)])
```

</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 3 – Åpningstrekk for hvit bonde</summary>

Bonden blir hindret fra å gå fremover hvis det står en annen brikke direkte foran, uansett hvordan farge den brikken
har. Vi kan bruke dette og `match` til å lage en enkel algoritme for å finne gyldige trekk.

Her følger en enkel kode for å finne bondens vanlige trekk (for å gå fremover, ikke for å slå andre brikker):

```rust
let (x, _) = self.position;
HashSet::from_iter([(x, 2), (x, 3)])
```

</details>