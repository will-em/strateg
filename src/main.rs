use std::fmt;

const BOARD_SIZE: usize = 8;
const BOARD_SIZE_U8: u8 = BOARD_SIZE as u8;

#[derive(Clone, Copy)]
enum Color { White, Black }

#[derive(Clone, Copy)]
enum Kind { Pawn, Knight, Bishop, Rook, Queen, King }


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Kind::Pawn   => "Pawn",
            Kind::Knight => "Knight",
            Kind::Bishop => "Bishop",
            Kind::Rook   => "Rook",
            Kind::Queen  => "Queen",
            Kind::King   => "King",
        })
    }
}

#[derive(Clone, Copy)]
struct Piece {
    color: Color,
    kind: Kind
}

impl Piece {
    fn glyph(self) -> char {
        use Color::*;
        use Kind::*;
        match (self.color, self.kind) {
            (White, King)   => '♔',
            (White, Queen)  => '♕',
            (White, Rook)   => '♖',
            (White, Bishop) => '♗',
            (White, Knight) => '♘',
            (White, Pawn)   => '♙',
            (Black, King)   => '♚',
            (Black, Queen)  => '♛',
            (Black, Rook)   => '♜',
            (Black, Bishop) => '♝',
            (Black, Knight) => '♞',
            (Black, Pawn)   => '♟',
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{} {}", self.color, self.kind)

    }
}

struct Square(u8);

impl Square {
    fn from_coords(file: u8, rank: u8) -> Option<Self> {
        (file < BOARD_SIZE_U8 && rank < BOARD_SIZE_U8)
            .then(|| Self(rank * BOARD_SIZE_U8 + file))
    }

    fn idx(&self) -> usize {
        self.0 as usize
    }
}

struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {

    fn empty() -> Self {
        Self{ squares: [None; 64] }
    }

        fn start_pos() -> Self {
        use Color::*;
        use Kind::*;

        let mut board = Self::empty();

        let sq = |file: u8, rank: u8| Square::from_coords(file, rank).unwrap();

        // Pawns
        for file in 0..8 {
            board.place(Piece { color: White, kind: Pawn }, sq(file, 1));
            board.place(Piece { color: Black, kind: Pawn }, sq(file, 6));
        }

        // White back rank
        let white_back_rank = [
            Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook
        ];
        for (file, &kind) in white_back_rank.iter().enumerate() {
            board.place(Piece { color: White, kind }, sq(file as u8, 0));
        }

        // Black back rank
        let black_back_rank = [
            Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook
        ];
        for (file, &kind) in black_back_rank.iter().enumerate() {
            board.place(Piece { color: Black, kind }, sq(file as u8, 7));
        }

        board
    }

    fn place(&mut self, p: Piece, sq: Square){
        self.squares[sq.idx()] = Some(p);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;

        for rank in (0..BOARD_SIZE_U8).rev() {
            write!(f, "{} |", rank + 1)?;

            for file in 0..BOARD_SIZE_U8 {
                let idx = (rank * BOARD_SIZE_U8 + file) as usize;
                let cell = match self.squares[idx] {
                    Some(p) => p.glyph(),
                    None    => ' ',
                };
                write!(f, " {} |", cell)?;
            }

            writeln!(f, " {}", rank + 1)?;
            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }

        writeln!(f, "    a   b   c   d   e   f   g   h")
    }
}

#[derive(Clone, Copy)]
enum MoveKind {
    Quiet,
    Capture,
    DoublePawnPush,
    EnPassant,
    Castle { kingside: bool },
    Promotion { to: Kind, is_capture: bool },
}

struct Move {
    from: Square,
    to: Square,
    kind: MoveKind,
}

struct SideRights {
    king: bool,   // O-O
    queen: bool,  // O-O-O
}

struct Castling {
    white: SideRights,
    black: SideRights,
}

struct Position {
    board: Board,
    stm: Color, // Side to move
    castling: Castling,
    ep: Option<Square>, // En passant
    halfmove: u16,
    fullmove: u16,
}

fn main() {
    let board = Board::start_pos();
    println!("{}", board);
}
