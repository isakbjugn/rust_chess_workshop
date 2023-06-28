# Oppgave 9
> **Mål:** Filtrere bort trekk som setter kongen i sjakk

> **Hvor skal jeg jobbe:** [board.rs](board.rs)

Nå skal vi ta i bruk `is_check()` metoden fra forrige oppgave, slik at vi forhindrer muligheten
for å sette seg selv i sjakk. Det innebærer også at dersom du står i sjakk, så er de eneste gyldige
trekkene de som enten flytter kongen til et felt som ikke er under angrep, eller blokkerer veien
til brikka som truer kongen. Sagt på en enklere måte så er gyldige trekk kun de som resulterer i en
stilling der kongen din ikke er i sjakk.

Dette kan vi implementere ved å utvide metoden `get_legal_squares()` i [board.rs](board.rs), ved å ta i bruk blant 
annet `is_check()`.

## Kjøring
```bash
cargo run 9
```
```bash
cargo test task_9
```

Du finner også hint i [hint.md](hint.md).