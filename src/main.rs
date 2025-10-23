enum Color { White, Black }

enum Kind { Pawn, Knight, Bishop, Rook, Queen, King }


impl Kind {

    fn to_string(self) -> String 
    {
        match self
        {
            Kind::Pawn => "Pawn".to_string(),
            Kind::Knight => "Knight".to_string(),
            Kind::Bishop => "Bishop".to_string(),
            Kind::Rook => "Rook".to_string(),
            Kind::Queen => "Queen".to_string(),
            Kind::King => "King".to_string(),
        }
    }
}

struct Piece {
    color: Color,
    kind: Kind
}

fn main() {
    let test_piece = Piece { color: Color::White, kind: Kind::King };
    println!("Hello, world! {:?}", test_piece.kind.to_string());
}
