# 6 – Vektor og iterator

<span style="justify-content: space-between; display: flex"><span>
    [← 5 struct og trait](./5-struct-og-trait.md)
</span> <span>
    [7 HashSet og HashMap →](./7-hashset-og-hashmap.md)
</span></span>

___

## Vec
`Vec` er en egnet datastruktur for å representere en rekke med felter der rekkefølgen har betydning.

En kan instansiere vektorer direkte fra en liste med makroen `vec!`:
```rust
let my_vec = vec![1, 2, 3, 4];
```

eller initialisere en tom vektor og legge til nye elementer med `.push`:
```rust
let mut my_vec = Vec::new();
my_vec.push(1);
```

Les mer om `Vec` i [Rust-boka](https://doc.rust-lang.org/book/ch08-01-vectors.html) og i
[Rust-dokumentasjonen om Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

## Iteratorer
Ofte er det nyttig å gjøre `Vec` eller andre datatyper om til iteratorer for å få tilgang til bestemte metoder, slik som
- `map()` som avbilder fra hvert element i datastrukturen til noe annet
- `filter()` som filtrerer hvert elemenet basert på en betingelse
- `rev()` som reverserer retningen på iteratoren
- `flat_map()` som produserer en iterator hvor hvert element, og deretter samler alle elementene i disse iteratorene
  til én felles iterator (og kollapser slik en iterator i en iterator)
- `cloned()` som tar deg fra iterator av `&T` til `T` (dette bruker vi for å lage f.eks. nye `Vec` av eksisterende `Vec`)
- `collect()` som samler en iterator til en ny datastruktur (som f.eks. `Vec` eller `HashSet`)

For å gjøre en `Vec` om til en iterator kan du bruke `iter()` eller `into_iter()`. Disse er forskjellige med hensyn til
hvilken iterator du får og hvem som får eierskap til den opprinnelige vektoren.

### iter og into_iter

Her er noe som er lett å bli forvirret av selv etter å ha jobbet med Rust en stund: Hva er forskjellen på `iter()` og
`into_iter()`, disse to metodene som gir deg iteratorer fra vektorer eller andre datastrukturer?

`iter` er en iterator til det eksisterende objektet, og inneholder derfor referanser. Det vil si,

```rust
let vec_1 = vec![1, 2, 3, 4, 5]; // Denne er av typen Vec<u8>
vec_1.iter() // Denne iteratoren holder elementer av typen &u8
// Vi kan fremdeles lese fra vec_1
```

Dersom du vil lagre elementene fra `vec_1.iter()` i eksempelet over i en ny vektor, må du bruke `cloned()` for å få en
iterator som holder `u8`.

`into_iter()` konsumerer et eksisterende objekt, og gir deg en iterator av elementene i det tidligere objektet. Det vil si,

```rust
let vec_1 = vec![1, 2, 3, 4, 5]; // Denne er av typen Vec<u8>
vec_1.into_iter() // Denne iteratoren holder elementer av typen u8
// Vi kan ikke lese fra vec_1, for verdiene ble flyttet ved into_iter
```

Oppsummert: `iter` _låner_ bare en verdi (gjennom en immutabel referanse), mens `into_iter()` _flytter_ verdiene inn i
en iterator, som innebærer at den opprinnelige vektoren ikke er tilgjengelig lenger. For å gjøre en iterator med
referanser må vi klone alle verdiene, som kan gjøres med `iter().cloned()`.

### cloned
`cloned` dukker ofte opp i iterator-sammenheng, og kan virke forvirrende, men ikke vær redd! Den er her for å hjelpe deg!
Når du holder på en iterator med referanser til elementer, og ønsker å samle disse elementene til en ny datastruktur,
kan du bruke `cloned` på hele iteratoren for å bytte `&T` til nye (klonede) `T`.

Dette betyr at `iter().cloned()` faktisk gir deg det samme som `into_iter()`, men i stedet for å flytte alle verdiene
inn i iteratoren, kloner vi objektet, uten å endre de opprinnelige verdiene. Dersom vi ikke skal bruke `vec_1` fra
eksempelet over videre, er det billigere å flytte enn å kopiere, og med `into_iter()` kommuniserer vi dessuten at vi
er ferdige med `vec_1`.

Merk: `iter().cloned()` er i prinsippet det samme som `iter().map(|&t| t.clone())`, men er ferdigimplementert for alle
iteratorer og enklere å kalle på.

### collect
`collect()` brukes på en iterator for å samle iteratorens elementer til en ny datastruktur (som f.eks. `Vec` eller `HashSet`)
Når du bruker `collect()` på en iterator kan Rust noen ganger utlede fra konteksten hvilken type du vil samle
iteratoren til, mens andre ganger er det nødvendig å oppgi type manuelt. Her har du to måter å gjøre det på:

Oppgi type når du tilordner:
```rust
let vec_1 = vec![1, 2, 3, 4, 5];
let vec_2: Vec<u8> = vec_1.iter().cloned().filter(|n| n > 2).collect();
assert_eq!(vec![3, 4, 5], vec_2)
```

Oppgi type ved samling (vec *collect*):
```rust
let vec_1 = vec![1, 2, 3, 4, 5];
let vec_2 = vec_1.iter().cloned().filter(|n| n > 2).collect::Vec<u8>();
assert_eq!(vec![3, 4, 5], vec_2)
```