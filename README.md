# Velkommen til Rust-workshop!
 ― Workshop for å lære programmeringsspråket Rust gjennom sjakk ―

<span style="justify-content: space-between; display: flex"><span>
    <strong>Er du klar for Rust-workshop? Hopp til intro-siden!</strong>
</span> <span>
    [Intro →](./doc/intro.md)
</span></span>

___

![](./images/ferris.png)

# Kom i gang
Den offisielt anbefalte installasjonsmetoden på macOS og Linux er via rustup scriptet. Du kan kjøre det slik:
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

For Windows, følg instruksjonene under [`Installing rustup on Windows` her](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows).

## Rust plugin i VS Code

Installer utvidelsen `rust-analyzer` i VS Code for å få støtte for Rust programmering.
For at debugging med breakpoints skal fungere kan du i tillegg installere utvidelsen CodeLLDB.

## Rust plugin i IntelliJ
Dersom du bruker IntelliJ anbefales det sterkt å installere Rust-utvidelsen, og konfigure den slik:

![](images/intellijconfig.png)

## Kjøre programmet

* `cargo run` for å kjøre programmet i oppgaven du jobber med
* `cargo run finished` for å gjøre det ferdige sjakk-spillet
* `cargo test` for å kjøre alle tester
* `cargo test task_0` kjører tester for en spesifikk oppgave (erstatt 0 med din oppgave)

Oppsettet for workshopen er beskrevet i [doc/workshop.md](./doc/workshop.md). Der finner du en full oversikt med alle
_run_- og _test_-kommandoene, som kan kjøres rett fra _.md_-filen i IntelliJ.
