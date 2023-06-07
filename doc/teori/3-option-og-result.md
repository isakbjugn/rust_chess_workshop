# 3 – Option og Result

<span style="justify-content: space-between; display: flex"><span>
    [← 2 Muterbarhet](./2-muterbarhet.md)
</span> <span>
    [4 match →](./4-match.md)
</span></span>

___

I Rust er `Option` og `Result` to typer som brukes for å håndtere potensielle fravær av verdier og feilhåndtering. De
gir en tryggere og mer robust tilnærming sammenlignet med å bruke null-verdier.

> Derfor eksisterer faktisk ikke `null` i Rust i det hele tatt!

## Option

Option er en `enum` i Rust som representerer enten et tilstedeværende (`Some`) eller fraværende (`None`) verdi.
Den brukes for å håndtere situasjoner der verdien kan være enten til stede eller fraværende, i stedet for å bruke 
null-verdier. Ved å bruke `Option`, tvinges koden til å være mer bevisst på håndtering av potensielle fravær av 
verdier og kan bidra til å unngå null-relaterte feil. Her er et eksempel som bruker `match` for å håndtere en `Option` 
som returneres når man forsøker å finne et bestemt element i en `Vec`:

```rust
fn main() {
    let most_loved_languages = vec!["rust", "elixir", "clojure", "typescript", "julia", "python", "delphi"];

    match most_loved_languages.iter().enumerate().find(|(_, &lang)| lang == "go") {
        Some((lang, index)) => println!("{} er det {}. høyest elskede språket", lang, index),
        None => println!("Språket er tydeligvis ikke særlig elsket"),
    }
}
```

I dette eksempelet er `find()` en iterator-metode som returnerer en `Option`, som vi bruker til å finne et bestemt 
element i vektoren. Dersom elementet finnes returneres `Some()`, som holder på verdien vi er interesserte i, hvis ikke 
returneres `None`, som ikke holder på noe. Vi kan deretter bruke `match` (se [4 – match](./4-match.md)) for å 
håndtere de to tilfellene.

## Result

Result er en `enum` i Rust som representerer enten en vellykket verdi (`Ok`) eller en feil (`Err`). `Result` likner 
`Option` men er litt annerledes: `Ok` holder på en gyldig verdi (resultatet), mens `Err` holder på en feilmelding.
`Result` brukes til å håndtere feilhåndtering og returnere resultater som kan være enten vellykkede eller mislykkede.
Her er et eksempel med `Result`:

```rust
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Division by zero"));
    }
    Ok(x / y)
}

fn main() {
    let result = divide(10, 2);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
```

I dette eksempelet definerer vi funksjonen `divide`, som utfører en divisjon og returnerer et `Result`-objekt. Legg 
merke til at returtypen til funksjonen er `Result<i32, String>`. `i32` er typen til den gyldige resultat-verdien, 
mens `String`  er verdien til feilmeldingen.