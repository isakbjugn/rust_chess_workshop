# 8 – clone og copy

<span style="justify-content: space-between; display: flex"><span>
    [← 7 HashSet og HashMap](./7-hashset-og-hashmap.md)
</span> <span>
    [Teorioversikt →](../teori.md)
</span></span>

___

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

### Clone og Copy som trait
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