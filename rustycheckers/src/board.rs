#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }

    pub fn crowned(p: GamePiece) -> GamePiece {
        GamePiece {
            color: p.color,
            crowned: true,
        }
    }
}

const PIECEFLAG_BLACK: u8 = 1;
const PIECEFLAG_WHITE: u8 = 2;
const PIECEFLAG_CROWN: u8 = 4;

impl Into<i32> for GamePiece {
    fn into(self) -> i32 {
        let mut val: u8 = 0;
        if self.color == PieceColor::Black {
            val += PIECEFLAG_BLACK;
        } else if self.color == PieceColor::White {
            val += PIECEFLAG_WHITE;
        }

        if self.crowned {
            val += PIECEFLAG_CROWN;
        }

        val as i32
    }
}


#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {

    pub fn on_board(self) -> bool {
        let Coordinate(x, y) = self; // (1)
        x <= 7 && y <= 7
    }


    // (2)
    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps = Vec::new();
        let Coordinate(x, y) = *self;
        if y >= 2 {
            jumps.push(Coordinate(x + 2, y - 2));
        }
        jumps.push(Coordinate(x + 2, y + 2));

        if x >= 2 && y >= 2 {
            jumps.push(Coordinate(x - 2, y - 2));
        }
        if x >= 2 {
            jumps.push(Coordinate(x - 2, y + 2));
        }
        jumps.into_iter()
    }

    // (3)
    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves = Vec::new();
        let Coordinate(x, y) = *self;
        if x >= 1 {
            moves.push(Coordinate(x - 1, y + 1));
        }
        moves.push(Coordinate(x + 1, y + 1));
        if y >= 1 {
            moves.push(Coordinate(x + 1, y - 1));
        }
        if x >= 1 && y >= 1 {
            moves.push(Coordinate(x - 1, y - 1));
        }
        moves.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
        Move {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}
