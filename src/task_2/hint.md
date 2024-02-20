# Hint for oppgave 2

## Hint som er nyttige

<details>
<summary>Hint 1 – Når er bonden i åpningsposisjon?</summary>

Ettersom bonden kun kan gå fremover og aldri bakover, så kan du være sikker på at

* Hvit bonde er i åpningsposisjon når `y = 1`

Kan du bruke dette for å avgjøre når bonden skal kunne gå to felt fremover, versus når den kun får gå ett felt?

</details>

<details>
<summary>Hint 2 – Hva med når en bonde står i veien?</summary>

- Hvilke åpningstrekk kan en bonde gjøre når det står en brikke to felt frem?
- Hvilke åpningstrekk er tilgjengelige når det står en brikke direkte foran bonden?
- Hvilke trekk er tilgjengelig etter åpningstrekket, når det står en brikke direkte foran?

Dette er tilfeller du burde ta hensyn til i koden.

</details>

<details>
<summary>Hint 3 – Les om match</summary>

Disse tre delene av workshop-teorien kan være spesielt nyttig i denne oppgaven:

* [match](../../doc/teori/4-match.md)
* [match og if](../../doc/teori/4-match.md#match-og-if)
* [Dobbel match](../../doc/teori/4-match.md#dobbel-match)

</details>

<details>
<summary>Hint 4 – Les mer om HashSet</summary>

Ta en titt på [HashSet](../../doc/teori/7-hashset-og-hashmap.md) i workshop-teorien. Spesielt operasjonen 
`HashSet::union()` kan være nyttige for denne oppgaven.

Du kan også lese mer om `HashSet` og disse metodene i
[Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html).

</details>

## Hint som avslører en mulig løsning

<details>
<summary>Hint 5 – En algoritme for bondetrekk</summary>

Bonden blir hindret fra å gå fremover hvis det står en annen brikke direkte foran, uansett hvordan farge den brikken
har. Vi kan bruke dette og `match` til å lage en enkel algoritme for å finne gyldige trekk.

Her følger en enkel kode for å finne bondens vanlige trekk (for å gå fremover, ikke for å slå andre brikker):

```rust
let (x, y) = self.position;
let other_pieces: HashSet<_> = team.union(rival_team).collect();
match y {
    _ if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
    1 if !other_pieces.contains(&(x, y + 2)) => HashSet::from_iter([(x, 2), (x, 3)]),
    _ => HashSet::from_iter([(x, y + 1)]),
}
```

</details>
