# 2 – Lånesystemet (_borrow checker_)

<span style="justify-content: space-between; display: flex"><span>
[← 1 Syntaks](./1-syntaks.md)
</span> <span>
[3 Option og Result →](./3-option-og-result.md)
</span></span>

___

Rusts _lånesystem_, kjent på engelsk som _borrow checker_, er en av de unike egenskapene ved språket. Det 
gir et sett med regler som sikrer minne- og trådsikkerhet uten behov for manuell minnehåndtering eller _garbage 
collection_. Lånesystemet forhindrer samtidig mutasjoner av data fra å føre til datakappløp og andre vanlige feil i 
parallell programmering.

## Eierskap
I Rust kan hver verdi kun ha én eier. En eier er variabelen eller datastrukturen som har ansvar for å frigjøre 
minnet knyttet til en verdi når verdien går ut av _scope_. Alle andre verdier må enten
 * _låne_ variabelen, dvs. bruke en immuterbar eller muterbar referanse, eller
 * overta eierskapet, gjennom flytting. Ved flytting blir den opprinnelige variabelen utilgjengelig.

```rust
let s1 = String::from("hei og hopp!");
let s2 = s1;

// Dette er ikke lov, fordi verdien ble flyttet fra `s1` til `s2`
println!("{}, world!", s1);
```

Dette mønsteret kalles _borrow after move_, og vil ikke la oss kompilere koden.

> Gjennom at kun én variabel har ansvar for å frigjøre minnet, unngår vi at to ulike variabler forsøker å frigi 
> samme minne, som er en kjent minnefeil (eng. _double free error_).

## Lån
Vi kan løse mange av vanskelighetene knyttet til eierskap og flytting ved å _låne_ en verdi. Vi bruker referanse til 
verdien, og ettersom referansen ikke eier verdien (kun låner den), kan referansen fint gå ut av _scope_ uten at det 
påvirker fremtidig bruk av verdien. Her er et eksempel på (immuterbar) låning:

```rust
fn print_without_change(curse_word: &String) {
    println!("{}, det var gode kanelboller!", curse_word)
    // Her går `curse_word` ut av scope
}

fn main() {
    let favorite_expression = String::from("Dæven");
    print_without_change(&favorite_expression);
    // verdien til favorite ble lånt bort, men er fremdeles tilgjengelig
    println!("Du burde ikke bruke ord som «{}»", favorite_expression)
}
```

Det finnes to retningslinjer for låning:
 * Du kan _enten_ ha én muterbar referanse, _eller_ så mange immuterbare referanser du vil
 * Referanser må alltid være gyldige (alltid peke til en verdi)

