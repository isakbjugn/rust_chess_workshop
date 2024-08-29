# Hint for oppgave 1

## Hint som er nyttige

<details>
<summary>Hint 1 – Når er bonden i åpningsposisjonen?</summary>

Hvit bonde er i åpningsposisjon når `y = 1`. Hva er da `y`-verdien til de to gyldige åpningstrekkene?

Dersom bonden ikke er i åpningsposisjonen, vet du at bonden kan flytte til feltet rett over (med mindre bonden står ved øverste rad av brettet).

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
let filled_hash_set = HashSet::from([(0, 0), (0, 1)])
```

</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 3 – Åpningstrekk for hvit bonde</summary>

Her følger en enkel kode for å finne bondens vanlige trekk (for å gå fremover, ikke for å slå andre brikker):

```rust
let (x, _) = self.position;
HashSet::from([(x, 2), (x, 3)])
```

</details>

<details>
<summary>Hint 4 – Generell bevegelse for hvit bonde</summary>

Her følger en enkel kode for å finne bondens vanlige trekk (for å gå fremover, ikke for å slå andre brikker):

```rust
let (x, y) = self.position;
HashSet::from([(x, y + 1)])
```

</details>

<details>
<summary>Hint 5 – Løsningsforslag</summary>

```rust
let (x, y) = self.position;
match y {
    1 => HashSet::from([(x, y + 1), (x, y + 2)]),
    7 => HashSet::new(),
    _ => HashSet::from([(x, y + 1)]
}
```

</details>