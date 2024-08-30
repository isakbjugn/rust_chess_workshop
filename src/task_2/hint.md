# Hint for oppgave 2

## Hint som er nyttige

<details>
<summary>Hint 1 – Hva med når en brikke står i veien?</summary>

- Hvilke åpningstrekk kan en bonde gjøre når det står en brikke to felt frem?
- Hvilke åpningstrekk er tilgjengelige når det står en brikke direkte foran bonden?
- Hvilke trekk er tilgjengelig etter åpningstrekket, når det står en brikke direkte foran?

Dette er tilfeller du burde ta hensyn til i koden.

</details>

<details>
<summary>Hint 2 – Les om match</summary>

Disse tre delene av workshop-teorien kan være spesielt nyttig i denne oppgaven:

* [match](../../doc/teori/4-match.md)
* [match og if](../../doc/teori/4-match.md#match-og-if)
* [Dobbel match](../../doc/teori/4-match.md#dobbel-match)

</details>

<details>
<summary>Hint 3 – Les mer om HashSet</summary>

Ta en titt på [HashSet](../../doc/teori/7-hashset-og-hashmap.md) i workshop-teorien. Spesielt operasjonene 
`HashSet::contains()` og `HashSet::union()` kan være nyttige for denne oppgaven.

Du kan også lese mer om `HashSet` og disse metodene i
[Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html).

</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 4 – Send inn et HashSet med andre brikkers posisjoner</summary>

I `get_moves` har vi allerede `team: &HashSet<(u8, u8)>` og `rival_team: &HashSet<(u8, u8)>`, altså posisjonene til våre egne og motstanderens brikker. Hvis vi ser et par linjer lengre ned, kan vi se at to `HashSet` blir slått sammen på denne måten:

```rust
forward_moves.union(&capture_moves).cloned().collect()
```

Dette kan vi også gjøre med `team` og `rival_team` slik at vi får ett `HashSet` vi kan sende som argument til `get_forward_moves`:

```rust
let other_pieces: HashSet<_> = team.union(rival_team).cloned().collect();

let forward_moves = self.get_forward_moves(&other_pieces);
```

Da må vi også oppdatere funksjonssignaturen til  `get_forward_moves` slik at den tar inn `other_pieces`:

```rust
fn get_forward_moves(&self, other_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)>
```

</details>

<details>
<summary>Hint 5 – En algoritme for bondetrekk</summary>

Bonden blir hindret fra å gå fremover hvis det står en annen brikke direkte foran, uansett hvordan farge den brikken
har. Vi kan bruke dette og `match` til å lage en enkel algoritme for å finne gyldige trekk.

Her følger en enkel kode for å finne bondens forovertrekk, med hensyn til hvor andre brikker står.

```rust
let (x, y) = self.position;
let other_pieces: HashSet<_> = team.union(rival_team).collect();
match y {
    _ if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
    1 if !other_pieces.contains(&(x, y + 2)) => HashSet::from([(x, 2), (x, 3)]),
    7 => HashSet::new(),
    _ => HashSet::from([(x, y + 1)])
}
```

</details>
