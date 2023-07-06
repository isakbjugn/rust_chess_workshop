use std::collections::HashSet;

pub trait Squares {
    fn as_board_positions(&self) -> HashSet<(u8, u8)>;
}

impl Squares for HashSet<(i8, i8)> {
    /// Returnerer et nytt `HashSet` med posisjonene som er innenfor brettet (filterer ut negative og >= 8)
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(x, y)| (0..8).contains(x) && (0..8).contains(y))
            .map(|(x, y)| (x as u8, y as u8))
            .collect()
    }
}

impl Squares for HashSet<(u8, u8)> {
    /// Returnerer et nytt `HashSet` med posisjonene som er innenfor brettet (filterer ut negative og >= 8)
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(x, y)| (0..8).contains(x) && (0..8).contains(y))
            .collect()
    }
}

impl Squares for [&str] {
    /// Returnerer et nytt `HashSet` med posisjonene som er innenfor brettet (filterer ut negative og >= 8)
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .map(|s| s.as_u8().unwrap())
            .collect()
    }
}

pub trait Square {
    fn as_i8(&self) -> Result<(i8, i8), &'static str>;
    fn as_u8(&self) -> Result<(u8, u8), &'static str>;
    fn as_string(&self) -> Result<String, &'static str>;
}

impl Square for (u8, u8) {
    fn as_i8(&self) -> Result<(i8, i8), &'static str> {
        Ok( (self.0 as i8, self.1 as i8) )
    }

    fn as_u8(&self) -> Result<(u8, u8), &'static str> {
        Ok(*self)
    }

    fn as_string(&self) -> Result<String, &'static str> {
        match (self.0, self.1) {
            (x, y) if (0..8).contains(&x) && (0..8).contains(&y) => {
                let file = (b'a' + self.0) as char;
                let rank = self.1 + 1;
                Ok(format!("{}{}", file, rank))
            }
            _ => Err("Feltet er ikke en gyldig sjakk-posisjon!")
        }
    }
}

impl Square for (i8, i8) {
    fn as_i8(&self) -> Result<(i8, i8), &'static str> {
        Ok(*self)
    }

    fn as_u8(&self) -> Result<(u8, u8), &'static str> {
        match (self.0, self.1) {
            (x, y) if x > 0 && y > 0 => Ok((x as u8, y as u8)),
            _ => Err("Feltet har negative koordinater!")
        }
    }

    fn as_string(&self) -> Result<String, &'static str> {
        self.as_u8()?.as_string()
    }
}

impl Square for &str {
    fn as_i8(&self) -> Result<(i8, i8), &'static str> {
        self.as_u8().map(|coordinate| (coordinate.0 as i8, coordinate.1 as i8))
    }

    fn as_u8(&self) -> Result<(u8, u8), &'static str> {
        if self.chars().count() != 2 { return Err("Feil antall tegn. Sjakkposisjoner har to tegn.") }
        let mut chars = self.chars();
        let file = chars.next().unwrap().to_ascii_lowercase() as i8 - 97;
        let rank = chars.next().unwrap() as i8 - 49;

        if (0..8).contains(&file) && (0..8).contains(&rank) {
            Ok((file as u8, rank as u8))
        } else {
            Err("Ugyldig sjakkposisjon. Linjer fra a–h og rader fra 1–8.")
        }
    }

    fn as_string(&self) -> Result<String, &'static str> {
        Ok(self.to_string())
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

#[macro_export]
macro_rules! empty_set {
    () => {
        {
            HashSet::<(u8, u8)>::new()
        }
    }
}

#[macro_export]
macro_rules! set {
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem($elem, $n).as_board_positions()
    );
    ($($x:expr),*) => (
        <[_]>::into_vec(Box::from_iter([$($x),*])).as_board_positions()
    );
    ($($x:expr,)*) => (
        vec![$($x),*].as_board_positions()
    )
}
