#![allow(unused)]

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}
#[derive(Clone, Copy)]
pub enum ChessPiece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl ChessPiece {
    fn to_string(&self) -> &str {
        match self {
            ChessPiece::Bishop(_) => "Bishop",
            ChessPiece::Pawn(_) => "Pawn",
            ChessPiece::Rook(_) => "Rook",
            ChessPiece::Knight(_) => "Knight",
            ChessPiece::Queen(_) => "Queen",
            ChessPiece::King(_) => "King",
        }
    }
    fn get_color(&self) -> &Color {
        match self {
            ChessPiece::Bishop(x) => x,
            ChessPiece::Pawn(x) => x,
            ChessPiece::Rook(x) => x,
            ChessPiece::Knight(x) => x,
            ChessPiece::Queen(x) => x,
            ChessPiece::King(x) => x,
        }
    }
}
#[derive(Clone)]
pub struct ChessBoard {
    pub turn: u32,
    pub board: [[Option<ChessPiece>; 8]; 8],
    pub black_king_pos: (usize, usize),
    pub white_king_pos: (usize, usize),
    pub removed_black: Vec<ChessPiece>,
    pub removed_white: Vec<ChessPiece>,
}

impl ChessBoard {
    pub fn create() -> Self {
        /// Creates an instance of the ChessBoard struct with the pieces in the starting positions, turn to one and empty removed piece vectors.
        Self {
            turn: 1,
            board: [
                [
                    Some(ChessPiece::Rook(Color::Black)),
                    Some(ChessPiece::Knight(Color::Black)),
                    Some(ChessPiece::Bishop(Color::Black)),
                    Some(ChessPiece::Queen(Color::Black)),
                    Some(ChessPiece::King(Color::Black)),
                    Some(ChessPiece::Bishop(Color::Black)),
                    Some(ChessPiece::Knight(Color::Black)),
                    Some(ChessPiece::Rook(Color::Black)),
                ],
                [
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                ],
                [
                    Some(ChessPiece::Rook(Color::White)),
                    Some(ChessPiece::Knight(Color::White)),
                    Some(ChessPiece::Bishop(Color::White)),
                    Some(ChessPiece::Queen(Color::White)),
                    Some(ChessPiece::King(Color::White)),
                    Some(ChessPiece::Bishop(Color::White)),
                    Some(ChessPiece::Knight(Color::White)),
                    Some(ChessPiece::Rook(Color::White)),
                ],
            ],
            white_king_pos: (4, 7),
            black_king_pos: (4, 0),
            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
        }
    }
    pub fn select_piece(
        &self,
        location: (usize, usize),
        faction: &Color,
    ) -> Option<Vec<(usize, usize)>> {
        /// Given the coordinates returns the selected pieces moves as an Option<Vec<(usize, usize)>> of possible coordinates.
        /// If a square with None values is picked returns None
        let (x, y) = location;

        let selected_piece = &self.board[y][x];
        let mut return_value: Option<Vec<(usize, usize)>> = None;
        match selected_piece {
            Some(piece) => match piece {
                ChessPiece::Pawn(color) => {
                    return_value = Some(self.generate_pawn_moves(x, y, color))
                }
                ChessPiece::Rook(color) => {
                    return_value = Some(self.generate_rook_moves(x, y, color))
                }
                ChessPiece::Knight(color) => {
                    return_value = Some(self.generate_knight_moves(x, y, color))
                }
                ChessPiece::Bishop(color) => {
                    return_value = Some(self.generate_bishop_moves(x, y, color))
                }
                ChessPiece::Queen(color) => {
                    return_value = Some(self.generate_queen_moves(x, y, color))
                }
                ChessPiece::King(color) => {
                    return_value = Some(self.generate_king_moves(x, y, color))
                }
            },
            None => return_value = None,
        }
        if let Some(value) = return_value {
            if selected_piece.clone().unwrap().get_color() == faction {
                Some(value)
            } else {
                None
            }
        } else {
            return_value
        }
    }
    pub fn set_piece(&mut self, source: (usize, usize), destination: (usize, usize)) {
        /// Given the original location of the piece (source) that is to be moved and the destination
        /// it removes any pieces at destination into on of the removed_{color} vectors
        /// and places the source piece at the destination position
        let selected_piece = &self.board[source.1][source.0];
        let selected_square = &self.board[destination.1][destination.0];

        if let Some(piece) = selected_square {
            match piece.get_color() {
                Color::Black => self.removed_black.push(piece.clone()),
                Color::White => self.removed_white.push(piece.clone()),
            }
        }
        if let Some(piece) = selected_piece {
            if let ChessPiece::King(color) = piece {
                match color {
                    Color::White => self.white_king_pos = destination,
                    Color::Black => self.black_king_pos = destination,
                }
            }

            self.board[destination.1][destination.0] = Some(selected_piece.clone().unwrap());
            self.board[source.1][source.0] = None;
        }
    }
    pub fn increase_turn(&mut self) {
        self.turn += 1;
    }
    pub fn faction_decider(&self) -> Color {
        if self.turn % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
    fn generate_king_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];
        if !((y as i32 - 1) < 0) {
            if !((x as i32 - 1) < 0) {
                let square = &self.board[y - 1][x - 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x - 1, y - 1))
                    }
                } else {
                    moves.push((x - 1, y - 1))
                }
            }
            if x + 1 < 8 {
                let square = &self.board[y - 1][x + 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x + 1, y - 1))
                    }
                } else {
                    moves.push((x + 1, y - 1))
                }
            }
            let square = &self.board[y - 1][x];
            if let Some(piece) = square {
                if piece.get_color() != color {
                    moves.push((x, y - 1))
                }
            } else {
                moves.push((x, y - 1))
            }
        }
        if y + 1 < 8 {
            if !((x as i32 - 1) < 0) {
                let square = &self.board[y + 1][x - 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x - 1, y + 1))
                    }
                } else {
                    moves.push((x - 1, y + 1))
                }
            }
            if x + 1 < 8 {
                let square = &self.board[y + 2][x + 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x + 1, y + 1))
                    }
                } else {
                    moves.push((x + 1, y + 1))
                }
            }
            let square = &self.board[y - 1][x];
            if let Some(piece) = square {
                if piece.get_color() != color {
                    moves.push((x, y + 1))
                }
            } else {
                moves.push((x, y + 1))
            }
        }
        if !((x as i32 - 1) < 0) {
            let square = &self.board[y][x - 1];
            if let Some(piece) = square {
                if piece.get_color() != color {
                    moves.push((x - 1, y))
                }
            } else {
                moves.push((x - 1, y))
            }
        }

        if x + 1 < 8 {
            let square = &self.board[y][x + 1];
            if let Some(piece) = square {
                if piece.get_color() != color {
                    moves.push((x + 1, y))
                }
            } else {
                moves.push((x + 1, y))
            }
        }
        moves
    }
    fn generate_queen_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        let mut counter: i32 = 1;
        let mut square = &self.board[y][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x + counter as usize, y));
                        break;
                    }
                    break;
                }

                None => moves.push((x + counter as usize, y)),
            }
            counter += 1;
            if (x + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y][x + counter as usize];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y][x - counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x - counter as usize, y));
                        break;
                    }
                    break;
                }

                None => moves.push((x - counter as usize, y)),
            }
            counter += 1;
            if (x as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y][x - counter as usize];
        }

        let mut counter: i32 = 1;
        let mut square = &self.board[y + counter as usize][x];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x, y + counter as usize));
                        break;
                    }
                    break;
                }
                None => moves.push((x, y + counter as usize)),
            }
            counter += 1;
            if (y + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y + counter as usize][x];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y - counter as usize][x];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x, y - counter as usize));
                        break;
                    }
                    break;
                }
                None => moves.push((x, y - counter as usize)),
            }
            counter += 1;
            if (y as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y - counter as usize][x];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y + counter as usize][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x + counter as usize, y + counter as usize));
                        break;
                    }
                    break;
                }

                None => moves.push((x + counter as usize, y + counter as usize)),
            }
            counter += 1;
            if (x + counter as usize) >= 8 || (y + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y + counter as usize][x + counter as usize];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y + counter as usize][x - counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x - counter as usize, y + counter as usize));
                        break;
                    }
                    break;
                }

                None => moves.push((x - counter as usize, y + counter as usize)),
            }
            counter += 1;
            if (x as i32 - counter) <= -1 || (y + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y + counter as usize][x - counter as usize];
        }

        let mut counter: i32 = 1;
        let mut square = &self.board[y - counter as usize][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x + counter as usize, y - counter as usize));
                        break;
                    }
                    break;
                }
                None => moves.push((x + counter as usize, y - counter as usize)),
            }

            counter += 1;
            if (x + counter as usize) >= 8 || (y as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y - counter as usize][x + counter as usize];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y - counter as usize][x - counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x - counter as usize, y - counter as usize));
                        break;
                    }
                    break;
                }

                None => moves.push((x - counter as usize, y - counter as usize)),
            }
            counter += 1;
            if (y as i32 - counter) <= -1 || (x as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y - counter as usize][x - counter as usize];
        }
        moves
    }
    fn generate_bishop_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        let mut counter: i32 = 1;
        let mut square = &self.board[y + counter as usize][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x + counter as usize, y + counter as usize));
                        break;
                    }
                    break;
                }

                None => moves.push((x + counter as usize, y + counter as usize)),
            }
            counter += 1;
            if (x + counter as usize) >= 8 || (y + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y + counter as usize][x + counter as usize];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y + counter as usize][x - counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x - counter as usize, y + counter as usize));
                        break;
                    }
                    break;
                }

                None => moves.push((x - counter as usize, y + counter as usize)),
            }
            counter += 1;
            if (x as i32 - counter) <= -1 || (y + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y + counter as usize][x - counter as usize];
        }

        let mut counter: i32 = 1;
        let mut square = &self.board[y - counter as usize][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x + counter as usize, y - counter as usize));
                        break;
                    }
                    break;
                }
                None => moves.push((x + counter as usize, y - counter as usize)),
            }

            counter += 1;
            if (x + counter as usize) >= 8 || (y as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y - counter as usize][x + counter as usize];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y - counter as usize][x - counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x - counter as usize, y - counter as usize));
                        break;
                    }
                    break;
                }

                None => moves.push((x - counter as usize, y - counter as usize)),
            }
            counter += 1;
            if (y as i32 - counter) <= -1 || (x as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y - counter as usize][x - counter as usize];
        }
        moves
    }
    fn generate_knight_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        if !((y as i32 - 2) < 0) {
            if !((x as i32 - 1) < 0) {
                let square = &self.board[y - 2][x - 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x - 1, y - 2))
                    }
                } else {
                    moves.push((x - 1, y - 2))
                }
            }
            if x + 1 < 8 {
                let square = &self.board[y - 2][x + 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x + 1, y - 2))
                    }
                } else {
                    moves.push((x + 1, y - 2))
                }
            }
        }
        if y + 2 < 8 {
            if !((x as i32 - 1) < 0) {
                let square = &self.board[y + 2][x - 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x - 1, y + 2))
                    }
                } else {
                    moves.push((x - 1, y + 2))
                }
            }
            if x + 1 < 8 {
                let square = &self.board[y + 2][x + 1];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x + 1, y + 2))
                    }
                } else {
                    moves.push((x + 1, y + 2))
                }
            }
        }
        if !((x as i32 - 2) < 0) {
            if !((y as i32 - 1) < 0) {
                let square = &self.board[y - 1][x - 2];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x - 2, y - 1))
                    }
                } else {
                    moves.push((x - 2, y - 1))
                }
            }
            if y + 1 < 8 {
                let square = &self.board[y + 2][x - 2];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x - 2, y + 1))
                    }
                } else {
                    moves.push((x - 2, y + 1))
                }
            }
        }

        if x + 2 < 8 {
            if !((y as i32 - 1) < 0) {
                let square = &self.board[y - 1][x + 2];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x + 2, y - 1))
                    }
                } else {
                    moves.push((x + 2, y - 1))
                }
            }
            if y + 1 < 8 {
                let square = &self.board[y + 1][x + 2];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        moves.push((x + 2, y + 1))
                    }
                } else {
                    moves.push((x + 2, y + 1))
                }
            }
        }

        moves
    }
    fn generate_rook_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        let mut counter: i32 = 1;
        let mut square = &self.board[y][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x + counter as usize, y));
                        break;
                    }
                    break;
                }

                None => moves.push((x + counter as usize, y)),
            }
            counter += 1;
            if (x + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y][x + counter as usize];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y][x - counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x - counter as usize, y));
                        break;
                    }
                    break;
                }

                None => moves.push((x - counter as usize, y)),
            }
            counter += 1;
            if (x as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y][x - counter as usize];
        }

        let mut counter: i32 = 1;
        let mut square = &self.board[y + counter as usize][x];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x, y + counter as usize));
                        break;
                    }
                    break;
                }
                None => moves.push((x, y + counter as usize)),
            }
            counter += 1;
            if (y + counter as usize) >= 8 {
                break;
            };
            square = &self.board[y + counter as usize][x];
        }
        let mut counter: i32 = 1;
        let mut square = &self.board[y - counter as usize][x];

        loop {
            match square {
                Some(piece) => {
                    if piece.get_color() != color {
                        moves.push((x, y - counter as usize));
                        break;
                    }
                    break;
                }
                None => moves.push((x, y - counter as usize)),
            }
            counter += 1;
            if (y as i32 - counter) <= -1 {
                break;
            };
            square = &self.board[y - counter as usize][x];
        }
        moves
    }
    fn generate_pawn_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        if matches!(color, Color::White) {
            if (y as i32 - 1) >= 0 {
                let front_piece = &self.board[y - 1][x];
                if matches!(front_piece, &Option::None) {
                    if y == 6 || y == 1 {
                        moves.push((x, y - 2))
                    }
                    moves.push((x, y - 1))
                }
                if (x as i32 - 1) >= 0 {
                    let fleft_piece = &self.board[y - 1][x - 1];
                    if matches!(fleft_piece, Some(_)) {
                        let piece = fleft_piece.as_ref().unwrap().get_color();
                        if let &Color::Black = piece {
                            moves.push((x - 1, y - 1))
                        }
                    }
                }
                if x + 1 < 8 {
                    let fright_piece = &self.board[y - 1][x + 1];
                    if matches!(fright_piece, Some(_)) {
                        let piece = fright_piece.as_ref().unwrap().get_color();
                        if let &Color::Black = piece {
                            moves.push((x + 1, y - 1))
                        }
                    }
                }
            }
        } else {
            if y + 1 < 8 {
                let front_piece = &self.board[y + 1][x];
                if matches!(front_piece, &Option::None) {
                    if (y == 6 || y == 1) && !(y > 7) {
                        moves.push((x, y + 2))
                    }
                    moves.push((x, y + 1))
                }
                if (x as i32 - 1) >= 0 {
                    let fleft_piece = &self.board[y + 1][x - 1];
                    if matches!(fleft_piece, Some(_)) {
                        let piece = fleft_piece.as_ref().unwrap().get_color();
                        if let &Color::White = piece {
                            moves.push((x - 1, y + 1))
                        }
                    }
                }
                if x + 1 < 8 {
                    let fright_piece = &self.board[y + 1][x + 1];
                    if matches!(fright_piece, Some(_)) {
                        let piece = fright_piece.as_ref().unwrap().get_color();
                        if let &Color::White = piece {
                            moves.push((x + 1, y + 1))
                        }
                    }
                }
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_creation_test() {
        let correct_board = [
            [
                Some(ChessPiece::Rook(Color::Black)),
                Some(ChessPiece::Knight(Color::Black)),
                Some(ChessPiece::Bishop(Color::Black)),
                Some(ChessPiece::Queen(Color::Black)),
                Some(ChessPiece::King(Color::Black)),
                Some(ChessPiece::Bishop(Color::Black)),
                Some(ChessPiece::Knight(Color::Black)),
                Some(ChessPiece::Rook(Color::Black)),
            ],
            [
                Some(ChessPiece::Pawn(Color::Black)),
                Some(ChessPiece::Pawn(Color::Black)),
                Some(ChessPiece::Pawn(Color::Black)),
                Some(ChessPiece::Pawn(Color::Black)),
                Some(ChessPiece::Pawn(Color::Black)),
                Some(ChessPiece::Pawn(Color::Black)),
                Some(ChessPiece::Pawn(Color::Black)),
                Some(ChessPiece::Pawn(Color::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(ChessPiece::Pawn(Color::White)),
                Some(ChessPiece::Pawn(Color::White)),
                Some(ChessPiece::Pawn(Color::White)),
                Some(ChessPiece::Pawn(Color::White)),
                Some(ChessPiece::Pawn(Color::White)),
                Some(ChessPiece::Pawn(Color::White)),
                Some(ChessPiece::Pawn(Color::White)),
                Some(ChessPiece::Pawn(Color::White)),
            ],
            [
                Some(ChessPiece::Rook(Color::White)),
                Some(ChessPiece::Knight(Color::White)),
                Some(ChessPiece::Bishop(Color::White)),
                Some(ChessPiece::Queen(Color::White)),
                Some(ChessPiece::King(Color::White)),
                Some(ChessPiece::Bishop(Color::White)),
                Some(ChessPiece::Knight(Color::White)),
                Some(ChessPiece::Rook(Color::White)),
            ],
        ];
        let test_board: ChessBoard = ChessBoard::create();
        for i in 0..8 {
            for j in 0..8 {
                let correct_state = match &correct_board[i][j] {
                    Some(x) => x.to_string(),
                    None => ".",
                };
                let test_state = match &test_board.board[i][j] {
                    Some(x) => x.to_string(),
                    None => ".",
                };
                assert_eq!(test_state, correct_state)
            }
        }
    }
    #[test]
    fn piece_pawn_white_test() {
        let board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            board: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::White)),
                    None,
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [
                    Some(ChessPiece::Pawn(Color::White)),
                    Some(ChessPiece::Pawn(Color::White)),
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::White)),
                    None,
                ],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 1,
            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
        };
        let coord = board
            .select_piece((3, 3), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(3, 2)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board
            .select_piece((6, 6), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(6, 4), (6, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board
            .select_piece((1, 6), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(0, 5), (2, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board
            .select_piece((0, 6), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(1, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_pawn_black_test() {
        let board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            board: [
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 2,
            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
        };
        let coord = board
            .select_piece((3, 3), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(3, 4)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board
            .select_piece((6, 1), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(6, 3), (6, 2)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_rook_test() {
        let board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            board: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Rook(Color::White)),
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 1,
            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
        };
        let correct_coord = vec![
            (5, 3),
            (6, 3),
            (7, 3),
            (3, 3),
            (2, 3),
            (1, 3),
            (0, 3),
            (4, 4),
            (4, 5),
            (4, 6),
            (4, 7),
            (4, 2),
            (4, 1),
            (4, 0),
        ];
        let coord = board
            .select_piece((4, 3), &board.faction_decider())
            .unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            board: [
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::White)),
                    None,
                    Some(ChessPiece::Rook(Color::White)),
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 1,
            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
        };
        let correct_coord = vec![
            (5, 3),
            (6, 3),
            (7, 3),
            (3, 3),
            (4, 4),
            (4, 5),
            (4, 6),
            (4, 7),
            (4, 2),
            (4, 1),
            // (4, 0),
        ];
        let coord = board
            .select_piece((4, 3), &board.faction_decider())
            .unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_knight_test() {
        let board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
            board: [
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    Some(ChessPiece::Bishop(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    Some(ChessPiece::Knight(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    Some(ChessPiece::Pawn(Color::White)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                ],
                [
                    None,
                    Some(ChessPiece::Knight(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ],
            turn: 2,
        };
        let coord = board
            .select_piece((3, 3), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![
            // (2, 1),
            (4, 1),
            (2, 5),
            (4, 5),
            (1, 2),
            (1, 4),
            (5, 2),
            (5, 4),
        ];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board
            .select_piece((1, 7), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![
            // (2, 1),
            (0, 5),
            (2, 5),
        ];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_king_test() {
        let mut board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
            board: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    Some(ChessPiece::King(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::White)),
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::King(Color::White)),
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 2,
        };
        let coord = board
            .select_piece((1, 3), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![
            (0, 2),
            (2, 2),
            (1, 2),
            (0, 4),
            (2, 4),
            (1, 4),
            (0, 3),
            (2, 3),
        ];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        board.increase_turn();
        let coord = board
            .select_piece((6, 5), &board.faction_decider())
            .unwrap();
        let correct_coord: Vec<(usize, usize)> =
            vec![(7, 4), (6, 4), (5, 6), (7, 6), (6, 6), (5, 5), (7, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_bishop_test() {
        let board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
            board: [
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::White)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Bishop(Color::White)),
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 1,
        };
        let correct_coord = vec![
            (5, 4),
            (6, 5),
            (7, 6),
            (3, 4),
            (2, 5),
            (1, 6),
            // (0, 7),
            (5, 2),
            (6, 1),
            // (7, 0),
            (3, 2),
            (2, 1),
            (1, 0),
        ];
        let coord = board
            .select_piece((4, 3), &board.faction_decider())
            .unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_queen_test() {
        let board = ChessBoard {
            white_king_pos: (5, 7),
            black_king_pos: (5, 0),

            removed_black: vec![ChessPiece::Pawn(Color::Black); 0],
            removed_white: vec![ChessPiece::Pawn(Color::White); 0],
            board: [
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::White)),
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    Some(ChessPiece::Pawn(Color::White)),
                    None,
                    Some(ChessPiece::Queen(Color::White)),
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    Some(ChessPiece::Pawn(Color::Black)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 1,
        };
        let correct_coord = vec![
            (5, 3),
            (6, 3),
            (7, 3),
            (3, 3),
            (4, 4),
            (4, 5),
            (4, 6),
            (4, 7),
            (4, 2),
            (4, 1),
            (5, 4),
            (6, 5),
            (7, 6),
            (3, 4),
            (2, 5),
            (1, 6),
            // (0, 7),
            (5, 2),
            (6, 1),
            // (7, 0),
            (3, 2),
            (2, 1),
            (1, 0),
        ];
        let coord = board
            .select_piece((4, 3), &board.faction_decider())
            .unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn set_piece_test() {
        let mut board = ChessBoard::create();
        board.set_piece((0, 6), (0, 4));
        let coord = board
            .select_piece((0, 4), &board.faction_decider())
            .unwrap();
        let correct_coord = vec![(0, 3)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }

    #[test]
    fn update_king_position_test() {
        let mut board = ChessBoard::create();
        board.set_piece((4, 7), (4, 6));
        assert_eq!(board.white_king_pos, (4, 6));
        board.set_piece(board.black_king_pos, (5, 4));
        assert_eq!(board.black_king_pos, (5, 4));
    }
}
