# 4 – match

<span style="justify-content: space-between; display: flex"><span>
    [← 3 Option og Result](./3-option-og-result.md)
</span> <span>
    [5 struct og trait →](./5-struct-og-trait.md)
</span></span>

___

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

## match og if
Du kan også kombinere en generell `match` med en mer spesifikk `if` for å spisse betingelsene:

```rust
match self.color {
    Color::White if self.position == some_value => {} // Spesiallogikk for hvit bonde
    Color::White => {} // Generell logikk for hvit bonde
    Color::Black if self.position == some_other_value => {} // Spesiallogikk for sort bonde
    Color::Black => {} // Generell logikk for sort bonde
}
```

## Dobbel match

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

En forskjell på `match` i rust og `when` i kotlin er at du i kotlin ikke kan destrukturere tupler inni when, men dette kan du i rust. Det gjør den enklere å bruke og mye kraftigere.