use std::fmt;


enum Color { White, Black }

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

struct Piece {
    color: Color,
    kind: Kind
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{} {}", self.color, self.kind)

    }
}

fn main() {
    let test_piece = Piece { color: Color::White, kind: Kind::King };
    println!("Hello, world! {}", test_piece);
}
