#![allow(unused)]
pub enum Color {
    White,
    Black,
}
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
pub struct ChessBoard {
    turn: u32,
    board: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    pub fn create() -> ChessBoard {
        // Creates an instance of the ChessBoard struct with the pieces in the starting positions

        ChessBoard {
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
        }
    }

    pub fn select_piece(&self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
        // Returns the selected pieces moves as an array of possible coordinates
        let selected_piece = &self.board[y][x];

        match selected_piece {
            Some(piece) => match piece {
                ChessPiece::Pawn(color) => Some(self.genrate_pawn_moves(x, y, color)),
                ChessPiece::Rook(color) => Some(self.gnerate_rook_moves(x, y, color)),
                ChessPiece::Knight(color) => Some(self.gnerate_knight_moves(x, y, color)),
                ChessPiece::Bishop(color) => Some(self.generate_bishop_moves(x, y, color)),
                ChessPiece::Queen(color) => todo!(),
                ChessPiece::King(color) => todo!(),
            },
            None => None,
        }
    }
    fn generate_bishop_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        let mut counter: i32 = 1;
        let mut square = &self.board[y + counter as usize][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
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
    fn gnerate_knight_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        if !((y as i32 - 2) < 0) {
            if !((x as i32 - 1) < 0) {
                let square = &self.board[y - 2][x - 1];
                if let Some(piece) = square {
                    if !matches!(piece.get_color(), color) {
                        moves.push((x - 1, y - 2))
                    }
                } else {
                    moves.push((x - 1, y - 2))
                }
            }
            if x + 1 < 8 {
                let square = &self.board[y - 2][x + 1];
                if let Some(piece) = square {
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
                        moves.push((x - 1, y + 2))
                    }
                } else {
                    moves.push((x - 1, y + 2))
                }
            }
            if x + 1 < 8 {
                let square = &self.board[y + 2][x + 1];
                if let Some(piece) = square {
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
                        moves.push((x - 2, y - 1))
                    }
                } else {
                    moves.push((x - 2, y - 1))
                }
            }
            if y + 1 < 8 {
                let square = &self.board[y + 2][x - 2];
                if let Some(piece) = square {
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
                        moves.push((x + 2, y - 1))
                    }
                } else {
                    moves.push((x + 2, y - 1))
                }
            }
            if y + 1 < 8 {
                let square = &self.board[y + 1][x + 2];
                if let Some(piece) = square {
                    if !matches!(piece.get_color(), color) {
                        moves.push((x + 2, y + 1))
                    }
                } else {
                    moves.push((x + 2, y + 1))
                }
            }
        }

        moves
    }
    fn gnerate_rook_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        let mut counter: i32 = 1;
        let mut square = &self.board[y][x + counter as usize];

        loop {
            match square {
                Some(piece) => {
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
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
                    if !matches!(piece.get_color(), color) {
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
    fn genrate_pawn_moves(&self, x: usize, y: usize, color: &Color) -> Vec<(usize, usize)> {
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
        };
        let coord = board.select_piece(3, 3).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(3, 2)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board.select_piece(6, 6).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(6, 4), (6, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board.select_piece(1, 6).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(0, 5), (2, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board.select_piece(0, 6).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(1, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_pawn_black_test() {
        let board = ChessBoard {
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
            turn: 1,
        };
        let coord = board.select_piece(3, 3).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(3, 4)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let coord = board.select_piece(6, 1).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(6, 3), (6, 2)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_rook_test() {
        let board = ChessBoard {
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
        let coord = board.select_piece(4, 3).unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
        let board = ChessBoard {
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
        let coord = board.select_piece(4, 3).unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_knight_test() {
        let board = ChessBoard {
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
            turn: 1,
        };
        let coord = board.select_piece(3, 3).unwrap();
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
        let coord = board.select_piece(1, 7).unwrap();
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
    fn select_bishop_test() {
        let board = ChessBoard {
            board: [
                [None, None, None, None, None, None, None, None],
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
                [None, None, None, None, None, None, None, None],
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
            (0, 7),
            (5, 2),
            (6, 1),
            (7, 0),
            (3, 2),
            (2, 1),
            (1, 0),
        ];
        let coord = board.select_piece(4, 3).unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
}
