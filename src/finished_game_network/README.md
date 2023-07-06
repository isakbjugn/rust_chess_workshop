## Sjakk med nettverksfunksjonalitet (WIP)

Dette er en alternativ versjon av det ferdige spillet, hvor du istedenfor å måtte spille på samme datamaskin, kan spille
mot hverandre over nettverket. Dette er implementert med bl.a TcpListener og TcpStream fra std::net.

Det kan kjøres slik som dette:
```
cargo run finished_network --server
cargo run finished_network --client
```

Den som starter spillet med `--server` argumenetet blir da hvit spiller.

Enn så lenge brukes lokal IP addresse både når du kjører som server og som klient, så for å kunne spille nå må den som kjører i klient modus manuelt legge inn ip-addressen til den som kjører i server-modus inn i koden. Du må trolig også åpne opp port 9001 for at det skal fungere.