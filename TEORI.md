## Litt Rust-teori
Dette er teori og hint som kan være nyttige på tvers av mange av oppgavene i workshopen. Bruk dem flittig om du står
fast, og konsulter gjerne [Rust-dokumentasjonen](https://doc.rust-lang.org/book/) om du vil lese mer:
- [Rust-boka](https://doc.rust-lang.org/book/) for en grunnleggende innføring
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) med kjørbar kode som illustrerer mange konsepter
- [Standardbiblioteket i Rust](https://doc.rust-lang.org/std/index.html)

Her er en oversikt over temaene i denne teori-delen:
1. [match](#match)
   1. [match og if](#match-og-if)
   2. [Dobbel match](#dobbel-match)
2. [Vec](#vec) 
3. [Iteratorer](#iteratorer)
   1. [iter og into_iter](#iter-og-intoiter)
   2. [cloned](#cloned)
   3. [collect](#collect)
4. [HashSet](#hashset)
5. [clone og copy](#clone-og-copy)
   1. [Sammenhengen mellom clone og copy](#sammenhengen-mellom-clone-og-copy)
   2. [Utlede Copy og Clone](#utlede-copy-og-clone)

## match
Du kjenner nok `if` godt fra mange programmeringsspråk, og kanskje `when` fra Kotlin, som lar oss kjøre kode eller
returnere basert på hvilke betingelser som er sanne. I Rust har vi også `if`, men ofte foretrekker vi `match`, som lar
deg sjekke mot en eller flere betingelser.

```rust
enum Color {
    White,
    Black
}

match self.color {
    Color::White => {} // Gjør logikk for hvit bonde
    Color::Black => {} // Gjør logikk for sort bonde
}
```

I `match`-uttrykket kalles hver betingelse og det etterfølgende uttrykket for `match`-armer. I Rust er alt _uttrykk_
(eng. _expressions_), som gjør at man enten kan ha kode som kjører i hver arm, eller bruke returverdien fra `match` til
tilordne en variabel:

```rust
let my_color = match self.color {
    Color::White => {} // Returner noe for hvit bonde
    Color::Black => {} // Returner noe for sort bonde
}
```

På grunn av at Rust er sterkt typet og statisk type-sjekket, er også `match` bestandig _uttømmende_, det vil si,
kompilatoren kan til enhver tid bestemme om du har sjekket alle muligheter. For eksempel vil

```rust
match self.color {
    Color::White => {} // Gjør logikk for hvit bonde
    Color::Black => {} // Gjør logikk for sort bonde
}
```

gi feilkode [`Match must be exhaustive [E0004]`](https://doc.rust-lang.org/error_codes/E0004.html), og faktisk enda mer
spesifikt [`non-exhaustive patterns: color::Color::Black not covered [E0004]`](https://doc.rust-lang.org/error_codes/E0004.html).
Hvis du er kjent med Kotlin vet du at dette kun er mulig når en bruker mønsteret _sealed class hierarchy_, mens i Rust
er det alltid tilgjengelig. Denne mønster-gjenkjenningen er en av Rusts store styrker.

Som en kvikkfiks kan en alltid erstatte med _fang alle_-mønsteret `_`, som matcher på alle andre betingelser enn de en
har skrevet eksplisitt:
```rust
match self.color {
    Color::White => {} // Gjør logikk for hvit bonde
    _ => {}            // Håndter resten
}
```

### match og if
Du kan også kombinere en generell `match` med en mer spesifikk `if` for å spisse betingelsene:

```rust
match self.color {
    Color::White if self.position == some_value => {} // Spesiallogikk for hvit bonde
    Color::White => {} // Generell logikk for hvit bonde
    Color::Black if self.position == some_other_value => {} // Spesiallogikk for sort bonde
    Color::Black => {} // Generell logikk for sort bonde
}
```

### Dobbel match

Det vi så i hint 3 kan også kombineres i en dobbel `match`. Det går ut på å sette variabler sammen i en tuppel
(`(var_1, var_2)`) og så matche på den:

```rust
match (self.color, self.position.1) {
    (Color::White, some_value) => {} // Spesiallogikk for hvit bonde
    (Color::White, _) => {} // Generell logikk for hvit bonde
    (Color::Black, some_other_value) => {} // Spesiallogikk for sort bonde
    (Color::Black, _) => {} // Generell logikk for sort bonde
}
```

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

## HashSet
En matematisk mengde, som i Rust er implementert som `HashSet`, kjennetegnes ved at elementenes rekkefølge ikke har
betydning, og at ingen elementer opptrer flere ganger. I metodene som beregner lovlige trekk for en brikke er det ofte
nyttig å bruke metoder knyttet til `HashSet`, blant annet:

* `HashSet::union`: Gir alle verdiene som finnes i to `HashSet`

Brukes på følgende måte
```rust
let set_1 = HashSet::from_iter([1, 2, 3]);
let set_2 = HashSet::from_iter([3, 4, 5]);
let set_union: HashSet<_> = set_1.union(&set_2).collect()
assert_eq(set_union, HashSet::from_iter([1, 2, 3, 4, 5]))
```

* `HashSet::difference`: Gir alle verdiene som er unike for ett `HashSet` sammenliknet med et annet

Brukes på følgende måte
```rust
let set_1 = HashSet::from_iter([1, 2, 3]);
let set_2 = HashSet::from_iter([3, 4, 5]);
let set_difference: HashSet<_> = set_1.difference(&set_2).collect()
assert_eq(set_union, HashSet::from_iter([1, 2]))
```

I tillegg har du
* `HashSet::difference`: Gir alle verdiene som er unike for ett `HashSet` sammenliknet med et annet
* `HashSet::symmetric_difference`: Gir alle verdiene som er unike for to `HashSet` og *ikke* finnes i begge

Les mer om `HashSet` og lær hvordan disse metodene brukes i [Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)

## clone og copy
Dette er også noe forvirrende, selv når en har jobbet med Rust en periode, men en veldig nyttig distinksjon som gjør
livet enklere. Veldig kort fortalt:

 * `copy` er en grunn kopi, kalles implisitt:
 * `clone` er en dypkopi, må kalles eksplisitt:

Ikke fullt så kort fortalt:

* `copy` gjelder primitive typer eller typer som består av utelukkende primitive typer. Operasjonen er billig, og
innebærer ikke at nytt minne blir allokert på _heap_-en. Spesifikt kan man bruke `copy` på alle typer som implementerer
`Copy`-`trait`-en, og man kan implentere `Copy` for alle `struct`-er som består av slike typer. Oftest er dette
primitive typer som `u8`, `char`, `bool` etc. Man kaller ikke `copy` eksplisitt, men denne metoden kalles implisitt når
man tilordner primitive typer (eller typer som implementerer `Copy`) eller flytter slike.

* `clone` er en dypkopi-operasjon som man må implementere eksplisitt for mer komplekse typer. Det er ofte en dyrere
operasjon, spesielt om den innebærer å allokere nytt minne på _heap_-en, og den må kalles på eksplisitt med `clone()`.
For å kunne bruke `clone()` på en type må man implementere `Clone` for typen.

### Sammenhengen mellom `clone` og `copy`
Som nevnt over er `clone` knyttet til en `trait` som heter `Clone`, og `copy` til `Copy`. Faktisk er det slik at `Copy`
er en _supertrait_ av `Clone`, og dette innebærer at enhver type som implementere `Copy` også må implementere `Clone`.
Hvis vi har tilgang på implisitte `copy` har vi selvfølgelig ikke behov for en dyrere `clone`, men `Clone` må være der
(i praksis vil et kall til `clone` bare bruke den billige `copy` i dette tilfellet).

### Utlede Copy og Clone

For enkle klasser kan vi ofte utlede `Clone` og `Copy` for enkle typer med `derive`:

```rust
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black
}
```

Her vil kompilatoren klage dersom vi kun utleder `Copy`, ettersom typen også må implementere `Clone`.