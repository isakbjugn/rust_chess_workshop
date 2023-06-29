
pub trait ChessTerm {
    fn translate(&self) -> &'static str;
}

impl ChessTerm for &'static str {
    fn translate(&self) -> &'static str {
        match self.rsplit("::").next().unwrap() {
            "Pawn" => "bonde",
            "Rook" => "tårn",
            "Knight" => "springar",
            "Bishop" => "laupar",
            "Queen" => "dronning",
            "King" => "konge",
            _ => "ukjent brikke"
        }
    }
}