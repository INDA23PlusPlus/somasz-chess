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
                ChessPiece::Pawn(Color::White) => Some(self.genrate_pawn_moves(x, y, Color::White)),
                ChessPiece::Pawn(Color::Black) => Some(self.genrate_pawn_moves(x, y, Color::Black)),
                ChessPiece::Rook(_) => Some(self.gnerate_rook_moves(x, y)),
                ChessPiece::Knight(_) => todo!(),
                ChessPiece::Bishop(_) => todo!(),
                ChessPiece::Queen(_) => todo!(),
                ChessPiece::King(_) => todo!(),
            },
            None => None,
        }
    }

    fn gnerate_rook_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        let mut counter: i32 = 1;
        let mut square = &self.board[y][x + counter as usize];

        while match self.decide_square_content_color(square) {
            Some(x) => !matches!(x, color),
            None => true,
        } {
            match square {
                Some(piece) => {
                    moves.push((x + counter as usize, y));
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

        while match self.decide_square_content_color(square) {
            Some(x) => !matches!(x, color),
            None => true,
        } {
            match square {
                Some(piece) => {
                    moves.push((x - counter as usize, y));
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

        while match self.decide_square_content_color(square) {
            Some(x) => !matches!(x, color),
            None => true,
        } || (y + counter as usize) < 8
        {
            match square {
                Some(piece) => {
                    moves.push((x, y + counter as usize));
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

        while match self.decide_square_content_color(square) {
            Some(x) => !matches!(x, color),
            None => true,
        } || (y as i32 - counter) > -1
        {
            match square {
                Some(piece) => {
                    moves.push((x, y - counter as usize));
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

    fn genrate_pawn_moves(&self, x: usize, y: usize, color: Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];

        if matches!(color, Color::White) {
            let front_piece = &self.board[y - 1][x];
            if matches!(front_piece, &Option::None) {
                if y == 6 || y == 1 {
                    moves.push((x, y - 2))
                }

                moves.push((x, y - 1))
            }
        } else {
            let front_piece = &self.board[y + 1][x];
            if matches!(front_piece, None) {
                if (y == 6 || y == 1) && !(y > 7) {
                    moves.push((x, y + 2))
                }
                if !(y > 7) {
                    moves.push((x, y + 1))
                }
            }
        }
        moves
        // if moves.len() != 0 {
        //     moves
        // }
    }

    fn decide_square_content_color(&self, square: &Option<ChessPiece>) -> Option<Color> {
        match square {
            Some(piece) => match piece.get_color() {
                Color::White => Some(Color::White),
                Color::Black => Some(Color::Black),
            },
            None => None,
        }
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
        let board = ChessBoard::create();
        let coord = board.select_piece(0, 6).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(0, 4), (0, 5)];
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
    #[test]
    fn select_pawn_black_test() {
        let board = ChessBoard {
            board: [
                [None, None, None, None, None, None, None, None],
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
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
            turn: 1,
        };
        let coord = board.select_piece(3, 3).unwrap();
        let correct_coord: Vec<(usize, usize)> = vec![(3, 4), (3, 5)];
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
        ];
        let coord = board.select_piece(4, 3).unwrap();
        for i in 0..correct_coord.len() {
            assert_eq!(coord[i], correct_coord[i]);
        }
    }
}
