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
}
pub struct ChessBoard {
    trun: u32,
    board: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    fn create() -> ChessBoard {
        // Creates an instance of the ChessBoard struct with the pieces in the starting positions

        ChessBoard {
            trun: 1,
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

    fn select_piece(&self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
        // Returns the selected pieces moves as an array of possible coordinates
        let selected_piece = &self.board[y][x];

        match selected_piece {
            Some(piece) => match piece {
                ChessPiece::Pawn(Color::White) => Some(self.genrate_pawn_moves(x, y, Color::White)),
                ChessPiece::Pawn(Color::Black) => todo!(),
                ChessPiece::Rook(_) => todo!(),
                ChessPiece::Knight(_) => todo!(),
                ChessPiece::Bishop(_) => todo!(),
                ChessPiece::Queen(_) => todo!(),
                ChessPiece::King(_) => todo!(),
            },
            None => None,
        }
    }
    fn genrate_pawn_moves(&self, x: usize, y: usize, color: Color) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![(0, 0); 0];
        let front_piece = &self.board[y][x].unwrap();

        if matches!(color, Color::White){
            if matches!(front_piece(Color), color) {

                if (y == 6 || y == 1) && !(y > 7) {
                        moves.push((x, y + 2))
                }
            }
        } else{
            if (y == 6 || y == 1) && !(y > 7) {
                moves.push((x, y -2))
            
        }
    }


            moves.push(())
        moves
        // if moves.len() != 0 {
        //     moves
        // }
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
        #[test]
        fn piece_select_test() {
            let board = ChessBoard::create();
            let piece = board.select_piece(0, 6);
            let correct_coord = vec![(0, 5), (0, 4)];
            match piece {
                Some(coord) => assert_eq!(coord, correct_coord),
                None => assert!(false),
            }
        }
    }
}
