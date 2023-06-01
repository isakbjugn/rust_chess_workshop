use std::collections::HashSet;

pub trait Squares {
    fn as_board_positions(&self) -> HashSet<(u8, u8)>;
}

impl Squares for HashSet<(i8, i8)> {
    /// Returnerer et nytt HashSet med posisjonene som er innenfor brettet (filterer ut negative og >= 8)
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .map(|(y, x)| (y as u8, x as u8))
            .collect()
    }
}

impl Squares for HashSet<(u8, u8)> {
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .collect()
    }
}

impl Squares for [&str] {
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .map(|s| s.as_u8().unwrap())
            .collect()
    }
}

pub trait Square {
    fn as_i8(&self) -> Option<(i8, i8)>;
    fn as_u8(&self) -> Option<(u8, u8)>;
    fn as_string(&self) -> String;
}

impl Square for (u8, u8) {
    fn as_i8(&self) -> Option<(i8, i8)> {
        Some( (self.0 as i8, self.1 as i8) )
    }

    fn as_u8(&self) -> Option<(u8, u8)> {
        Some(*self)
    }

     fn as_string(&self) -> String {
         let file = ('a' as u8 + self.0) as char;
         let rank = self.1 + 1;
         return format!("{}{}", file, rank).to_string()
     }
}

impl Square for &str {
    fn as_i8(&self) -> Option<(i8, i8)> {
        self.as_u8().map(|coordinate| (coordinate.0 as i8, coordinate.1 as i8))
    }

    fn as_u8(&self) -> Option<(u8, u8)> {
        if self.chars().count() != 2 { return None }
        let mut chars = self.chars();
        let col = chars.next().unwrap().to_ascii_lowercase() as i8 - 97;
        let row = chars.next().unwrap() as i8 - 49;

        if (0..8).contains(&col) && (0..8).contains(&row) {
            return Some((row as u8, col as u8));
        }
        None
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

pub trait MoveDirection {
    fn filter_blocked_squares(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)>;
}

impl MoveDirection for Vec<(u8, u8)> {
    fn filter_blocked_squares(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let mut moves = HashSet::new();
        for square in self {
            if team.contains(square) {
                break
            }
            moves.insert(*square);
            if rival_team.contains(square) {
                return moves
            }
        }
        moves
    }
}
