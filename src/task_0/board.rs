pub struct Board;

impl Board {
    pub fn new() -> Board {
        Board
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        vec![vec!['_'; 8]; 8]
    }

    pub fn print(&self) {
        let board = self.create_board();

        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for _ in row.iter() {
                print!("|   ")
            }
            println!("|")
        }
        println!("   {:-<33}", "");
        println!("     A   B   C   D   E   F   G   H");
    }
}
