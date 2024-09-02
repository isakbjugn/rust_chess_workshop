# 1 – Syntaks i Rust

<span style="justify-content: space-between; display: flex"><span>
    [← Teorioversikt](../teori.md)
</span> <span>
    [2 Lånesystemet →](./2-borrow-checker.md)
</span></span>

___

## Deklarere variabler

I Rust er variabler immuterbare som standard, noe som betyr at de ikke kan endres etter at de er blitt tildelt en 
verdi. Dersom vi ønsker muterbare variabler, må dette tydeliggjøres med nøkkelordet `mut`. Her deklarerer vi en 
immuterbar og en muterbar variabel:

```rust
let x = 10;
// x = 20; // Feil: Kan ikke endre en immuterbar variabel
println!("x: {}", x);

let mut y = 5;
y = 10; // Muterbar variabel kan endres
println!("y: {}", y);
```

I tillegg finnes konstanter, som deklareres med nøkkelordet `const`. Disse er virkelig _konstanter_, det vil si  
ikke bare immuterbare (som `const` i JavaScript`, men de har en kjent, entydig verdi ved kompileringstid.

```rust
use chrono::prelude::*;

const DECEMBER: &str = "desember";

// Dette er ikke en gyldig konstant, for verdien er ikke kjent ved kompileringstid
const TODAY: DateTime<Local> = Local::now();
```

## Referanser
Også referanser er immuterbare som standard, og må deklareres som muterbare med `&mut`. Her er to eksempler på 
funksjoner som bruker de to typene referanser:

```rust
fn print_immutable_value(value: &i32) {
    println!("Immuterbar referanse: {}", value);
}

fn modify_mutable_value(value: &mut i32) {
    *value += 10;
    println!("Muterbar referanse: {}", value);
}

fn main() {
    let mut value = 5;

    print_immutable_value(&value);      // Skriver ut: 5
    modify_mutable_value(&mut value);   // Skriver ut: 15
    print_immutable_value(&value);      // Skriver ut: 15
}
```

 * `print_immutable_value` tar en immuterbar referanse (`&i32`) som parameter. Det tillater oss å lese verdien av 
   `value`, men ikke endre den. Vi skriver ut verdien i funksjonen uten å modifisere den.
 * `modify_mutable_value`tar en muterbar referanse (`&mut i32`) som parameter. Det gir oss muligheten til å endre 
   verdien av `value`. Vi øker verdien med `10` ved å bruke `*value += 10` og skriver ut den oppdaterte verdien.

## Uttrykk vs. utsagn
Rust er i hovedsak et uttrykk-basert språk (eng. _expressions_). Det betyr at de fleste operasjoner evaluerer til en verdi, f.eks.

```rust
let y = {
    let x = 9;
    x + 1
}
```

Her vil `x + 1` evalueres som uttrykk, og ettersom dette er siste uttrykk i blokken, vil `y` tilordnes denne verdien.

Dette har blant annet to praktiske konsekvenser:

> **`return` er ofte unødvendig**  
> Vi trenger altså `return` sist i en funksjon, fordi siste linje i en funksjon alltid vil evalueres som uttrykk og returneres fra funksjonen.
> 
> Vi kan derimot bruke `result` dersom vi ønsker å returnere _tidlig_ fra funksjonen, f.eks. i et «feil tidlig»-scenario.

> **Tærnær `if`**  
> Ettersom alle `if`-setninger evalueres som uttrykk, er det ingen forskjell på `if` som kontrollstruktur («basert på en betingelse vil jeg _gjøre_ det ene eller andre») og `if` som betinget tilordning («basert på en betingelse vil jeg _sette_ den ene eller andre verdien»). Vi kan med andre ord skrive
> ```rust
> let three_is_odd = if 3 % 2 true else false;

Rust har også utsagn (eng. _statements_), men disse tjener i hovedsak til  å _inneholde_ uttrykk og å styre rekkefølge på evaluering av uttrykk.