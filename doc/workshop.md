# Workshop

<span style="justify-content: space-between; display: flex"><span>
    [← Teorioversikt](./teori.md)
</span> <span>
    [Oppgave 0 →](../src/task_0/oppgave.md)
</span></span>

___

## Oppsett for workshopen
Dette er en selvdreven, testdreven workshop. Det overordnede målet er å bli kjent med (og glad i) Rust, og måten vi
gjør dette på er å bruke språket til å implementere sjakk. Workshopen er delt opp i en rekke oppgaver, som har hver sin
oppgavebeskrivelse og et sett med tester som først vil kjøre <span style="color: red">rødt</span>, men når du har løst
oppgaven riktig vil de kjøre <span style="color: green">grønt</span>.

I hver oppgave-mappe finner du en _oppgave.md_-fil, for eksempel [task_0/oppgave.md](../src/task_0/oppgave.md) for 
oppgave 0, som inneholder oppgavebeskrivelse, tips, og ofte relevante forklaringer rundt sjakkregler og det du skal 
implementere.

> Oppgavene er utformet for å løses i kronologisk rekkefølge, men om du står fast eller bare har lyst, **kan du alltid hoppe
til neste oppgave**. Der finner du dessuten en ferdigimplementert løsning av forrige oppgave.

Hver oppgave har i tillegg
et sett med hint (som f.eks. i [task_1/hint.md](../src/task_1/hint.md)), som både inneholder sparsomme tips som lar deg
finne ut mest mulig selv, og også hele løsningsforslag.

### Merk: Oppgave 0 er annerledes
I [oppgave 0](../src/task_0/oppgave.md) er hovedinnholdet i testen utkommentert, og du må selv fjerne 
kommentar-tegnene når du er klar til å teste koden din.
> Dette skyldes at oppgaven går ut på å deklarere en `struct` og en `enum`, og Rust kan ikke kompilere dersom
> vi i testen refereres til noe som ikke finnes (da kan vi ikke kjøre testen i det hele tatt).

## Kjøre programmet
`cargo run` vil først prøve å kjøre alle testene, og bruke dette til å kjøre `main`-metoden i oppgaven du jobber med for
øyeblikket. Det vil si,

* En av testene i `task_0` feiler → `task_0`-koden kjøres
* Alle testene i `task_0` kjører grønt → `task_1`-koden kjøres
* Alle testene i workshopen grønt vil → `finished_game` kjøres

Hvis du vil kjøre koden fra `task_0` igjen kan du gjøre det med `cargo run 0`.

### Kjør programmet i konteksten av en bestemt oppgave
Du kan også kjøre spillet med den koden du har implementert i en bestemt oppgave, ved å spesifisere oppgavenummeret når
du kjører _cargo_. Her har du en list over kommendoene for å kjøre hver av oppgavene:

* `cargo run 0`
* `cargo run 1`
* `cargo run 2`
* `cargo run 3`
* `cargo run 4`
* `cargo run 5`
* `cargo run 6`
* `cargo run 7`
* `cargo run 8`
* `cargo run 9`
* `cargo run 10`
* `cargo run 11`
* `cargo run 12`
* `cargo run finished`

Disse kan du teste allerede nå! Prøv å kjøre spillet i en bestemt oppgave, og se at sjakkbrettet blir tegnet opp! For
`finished game` vil du dessuten kunne flytte alle brikkene.

## Kjøre tester
`cargo test` brukes for å kjøre alle testene i prosjektet. Du kan dessuten snevre inn hvilke tester som kjøres ved å
spesifisere en del av navnet på testen(e) du vil kjøre, på denne måten:

* `cargo test task_0`
* `cargo test task_1::`
* `cargo test task_2`
* `cargo test task_3`
* `cargo test task_4`
* `cargo test task_5`
* `cargo test task_6`
* `cargo test task_7`
* `cargo test task_8`
* `cargo test task_9`
* `cargo test task_10`
* `cargo test task_11`
* `cargo test task_12`
* `cargo test finished`

Om du kjører disse nå, vil du se at alle testene fra `task_0` til og med `task_9` kjører
<span style="color: red">rødt</span>, mens testene i `finished_game` kjører <span style="color: green">grønt</span>.

## Litt om mappe- og filstruktur
Hver oppgave har generelt filstrukturen
```
├── task_x
│   ├── piece
│   │   ├── brikke_du_skal_implementere.rs
│   │   ├── brikke_fra_forrige_oppgave.rs
│   ├── board.rs
│   ├── hint.md
│   ├── mod.rs
│   ├── oppgave.md
```

Generelt vil filene du skal skrive kode i finnes i `piece`-katalogen. Fra og med oppgave 5 finnes også enda en brikke her,
nemlig den du implementerte i forrige oppgave (ofte kan denne være nyttig å se på).

Du vil kanskje også legge merke til at ikke alle filer blir med til neste oppgave. Når vi ikke lenger trenger en fil i
oppgaven vi jobber med, importerer vi den heller fra `finished_game`. Dette er for å unngå kodeduplikat, og for å kutte
kompleksiteten for hver oppgave. Du er alltid velkommen til å se i `finished_game` dersom du vil se nærmere på noen av
filene, eller dersom du vil ha et løsningsforslag.

## Litt om sjakkregler
I hver oppgave hvor du skal implementere trekk for ulike brikker, vil du også finne forklaring for reglene som 
gjelder den brikken.

> **NB! Om posisjoner**  
> Det er verdt å merke seg at posisjoner i dette sjakkspillet er null-indekserte. Det vil si at feltet nede i 
> venstre hjørne har verdien `(0, 0)` i koden, og `a1` i sjakk-domenet.
