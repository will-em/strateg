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

fn main() {
    let test_piece = Piece { color: Color::White, kind: Kind::King };
    let square = Square::from_coords(4, 0).unwrap();
    let mut board = Board::empty();
    board.place(test_piece, square);
    println!("Hello, world! {}", test_piece);
    println!();
    println!("{}", board);
}
