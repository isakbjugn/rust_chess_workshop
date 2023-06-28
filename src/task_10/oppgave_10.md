# Oppgave 10
> **Mål:** Kunne finne ut om kongen er sjakk matt

> **Hvor skal jeg jobbe:** [board.rs](board.rs)

Nå som vi har `is_check()` og klarer å ta hensyn til trekk som ville satt kongen i sjakk i
`get_legal_squares()`, kan vi nå faktisk implementere sjakkmatt!

Kongen er i sjakkmatt når han står i sjakk, og det ikke finnes noen trekk som kan forhindre at
kongen fortsatt står i sjakk. Du skal implementere `is_checkmate()` i `board.rs`.

## Kjøring
```bash
cargo run 10
```
```bash
cargo test task_10
```


Du finner også hint i [hint.md](hint.md).