# 5 – struct og trait

<span style="justify-content: space-between; display: flex"><span>
    [4 match →](./4-match.md)
</span> <span>
    [6 Vektor og iterator →](./6-vektor-og-iterator.md)
</span></span>

___

I Rust er en `struct` en datastruktur som tillater deg å definere og organisere relaterte verdier under ett navn. Den lar
deg kombinere flere felter med forskjellige typer i en enkelt enhet. Man definerer en `struct` på slik:

```rust
struct Company {
    workers: u8,
    address: String,
    using_rust: bool
}
```

Her er `workers`, `address` og `using_rust` alle eksempler på _felt_ (eng. _field_) knyttet til strukten. `Company` kan vi nå
instansiere på denne måten:

```rust
let sparebank_1_utvikling = Company {
    workers: 400,
    address: String::from("Hammersborggata 9a"),
    using_rust: false
};
```

## Implementere struct
En `struct` kan også ha en tilhørende implementasjon, hvis vi vil skrive funksjoner som tilhører `struct`-en. Dette gjøres
med en implementasjonsblokk på denne måten:

```rust
impl Company {
    pub fn new(address: &str, cool: bool) -> Company {
        Company { workers: 1, address: address.to_string(), using_rust: cool }
    }
    pub fn does_someone_know_rust(&self) -> bool {
        self.using_rust || self.workers > 100
    }
    fn hire_new_worker(&mut self) {
        self.workers += 1
    }
}
```

Legg merke til her at vi har flere typer metoder:
 * `new` er offentlig, tar ingen `&self` som argument, og returnerer `Company`
 * `does_someone_know_rust` er offentlig, tar `&self` som argument, returnerer en boolsk verdi
 * `hire_new_worker` er privat, tar `&mut self`, returnerer ingenting

`&self` og `&mut self` kaller vi _mottakere_ (eng. _receiver_) i Rust, og de forteller oss at funksjonene er _metoder_, som
opererer på eller fra en bestemt instans av `Company`. Funksjonene som ikke tar `self` er ikke metoder, men _assosierte
funksjoner_, og kan kalles fra med `Company::new()`, altså uten en bestemt instans. `&mut self` forteller at metoden
også muterer på selve objektet. Dersom vi ikke deklarerer objektet med `mut` kan vi heller ikke kalle på noen av
metodene som tar `&mut self` som argument.

> Om `struct` og `class`  
> Du har kanskje erfaring med C++, der `struct` er en datatype uten assosierte metoder, mens `class` er en type som også
> kan ha tilhørende metoder. I Rust skilles det ikke mellom disse, men du behøver ikke å ha en `impl`-block for en
> `struct`, og på den måten oppnår vi det samme som `struct` i C++.

## trait
I Rust er et `trait` en mekanisme som tillater deg å definere felles egenskaper og funksjonalitet som kan deles av flere
typer – enten strukter (som `Company`), eller datastrukturer som holder på strukter (f.eks. `Vec<Company>`).
Du kan tenke på et `trait` som et sett av regler eller en kontrakt som typer kan overholde. `trait` er svært likt
Kotlins `interface`, men du kan ikke definerere felt i en `trait`, selv ikke abstrakte felt, slik man kan med abstrakte
egenskaper (eng. _abstract properties_) i Kotlin.

Her definerer vi en `trait`, og implementerer den for `Company`:

```rust
trait SFDRCompliant {
    fn must_report_according_to_sfdr(&self) -> bool;
}

impl SFDRCompliant for Company {
    fn must_report_according_to_sfdr(&self) -> bool {
        self.workers > 500
    }
}
```