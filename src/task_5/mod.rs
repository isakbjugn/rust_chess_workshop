mod piece;
mod game;
mod board;

/// # Oppgave 5
/// 
/// I denne oppgaven skal vi implementere trekkene til tårnet.
///
/// Du skal implementere funksjoner i filen [./piece/rook.rs](./piece/rook.rs). Der finner du
/// en oppgavebeskrivelse, og kommentarer som forklarer hva de ulike metodene skal gjøre.
///
/// Du finner også hint i [./hint.md](/hint.md).
pub fn main() {
    println!("Køyrer game::main() i task_4");
    game::main();

    let vec_1 = vec![1, 2, 3, 4, 5]; // Denne er av typen Vec<u8>
    let iter_1 = vec_1.iter(); // Denne iteratoren holder elementer av typen &u8
    for i in vec_1 {
        println!("{i}")
    }

    let vec_2 = vec![1, 2, 3, 4, 5]; // Denne er av typen Vec<u8>
    vec_1.into_iter(); // Denne iteratoren holder elementer av typen &u8
    for i in vec_1 {
        println!("{i}")
    }

}
