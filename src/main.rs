use std::fmt;

const BOARD_SIZE: usize = 8;
const BOARD_SIZE_U8: u8 = BOARD_SIZE as u8;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

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
            Kind::Pawn => "Pawn",
            Kind::Knight => "Knight",
            Kind::Bishop => "Bishop",
            Kind::Rook => "Rook",
            Kind::Queen => "Queen",
            Kind::King => "King",
        })
    }
}

#[derive(Clone, Copy)]
struct Piece {
    color: Color,
    kind: Kind,
}

impl Piece {
    fn glyph(self) -> char {
        use Color::*;
        use Kind::*;
        match (self.color, self.kind) {
            (White, King) => '♔',
            (White, Queen) => '♕',
            (White, Rook) => '♖',
            (White, Bishop) => '♗',
            (White, Knight) => '♘',
            (White, Pawn) => '♙',
            (Black, King) => '♚',
            (Black, Queen) => '♛',
            (Black, Rook) => '♜',
            (Black, Bishop) => '♝',
            (Black, Knight) => '♞',
            (Black, Pawn) => '♟',
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.color, self.kind)
    }
}

#[derive(Clone, Copy, Debug)]
struct Square(u8);

impl Square {
    fn from_coords(file: u8, rank: u8) -> Option<Self> {
        (file < BOARD_SIZE_U8 && rank < BOARD_SIZE_U8).then(|| Self(rank * BOARD_SIZE_U8 + file))
    }

    fn idx(&self) -> usize {
        self.0 as usize
    }

    fn file(&self) -> u8 {
        self.0 % BOARD_SIZE_U8
    }

    fn rank(&self) -> u8 {
        self.0 / BOARD_SIZE_U8
    }

    fn offset_if_valid(&self, dx: i8, dy: i8) -> Option<Self> {
        let new_file = self.file() as i8 + dx;
        let new_rank = self.rank() as i8 + dy;

        if (0..BOARD_SIZE as i8).contains(&new_file) && (0..BOARD_SIZE as i8).contains(&new_rank) {
            Some(Self::from_coords(new_file as u8, new_rank as u8).unwrap())
        } else {
            None
        }
    }
}

struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    fn empty() -> Self {
        Self {
            squares: [None; 64],
        }
    }

    fn start_pos() -> Self {
        use Color::*;
        use Kind::*;

        let mut board = Self::empty();

        let sq = |file: u8, rank: u8| Square::from_coords(file, rank).unwrap();

        // Pawns
        for file in 0..8 {
            board.place(
                Piece {
                    color: White,
                    kind: Pawn,
                },
                sq(file, 1),
            );
            board.place(
                Piece {
                    color: Black,
                    kind: Pawn,
                },
                sq(file, 6),
            );
        }

        // White back rank
        let white_back_rank = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        for (file, &kind) in white_back_rank.iter().enumerate() {
            board.place(Piece { color: White, kind }, sq(file as u8, 0));
        }

        // Black back rank
        let black_back_rank = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        for (file, &kind) in black_back_rank.iter().enumerate() {
            board.place(Piece { color: Black, kind }, sq(file as u8, 7));
        }

        board
    }

    fn place(&mut self, p: Piece, sq: Square) {
        self.squares[sq.idx()] = Some(p);
    }

    fn piece(&self, square: Square) -> Option<Piece> {
        self.squares[square.idx()]
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
                    None => ' ',
                };
                write!(f, " {} |", cell)?;
            }

            writeln!(f, " {}", rank + 1)?;
            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }

        writeln!(f, "    a   b   c   d   e   f   g   h")
    }
}

#[derive(Clone, Copy, Debug)]
enum MoveKind {
    Quiet,
    Capture,
    DoublePawnPush,
    EnPassant,
    Castle { kingside: bool },
    Promotion { to: Kind, is_capture: bool },
}

#[derive(Debug)]
struct Move {
    from: Square,
    to: Square,
    kind: MoveKind,
}

struct SideRights {
    king: bool,  // O-O
    queen: bool, // O-O-O
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

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

const KING_OFFSETS: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

const QUEEN_DIRS: [(i8, i8); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn leaper_pseudo_moves(
    board: &Board,
    from: Square,
    color: Color,
    offsets: &[(i8, i8)],
) -> Vec<Move> {
    offsets
        .into_iter()
        .filter_map(|&(dx, dy)| from.offset_if_valid(dx, dy))
        .filter_map(|to| match board.piece(to) {
            Some(other_piece) if other_piece.color != color => Some(Move {
                from,
                to,
                kind: MoveKind::Capture,
            }),
            None => Some(Move {
                from,
                to,
                kind: MoveKind::Quiet,
            }),
            _ => None,
        })
        .collect()
}

fn slider_psuedo_moves(board: &Board, from: Square, color: Color, dirs: &[(i8, i8)]) -> Vec<Move> {
    dirs.into_iter()
        .flat_map(|&(dx, dy)| slider_ray(board, from, color, dx, dy))
        .collect()
}

fn slider_ray(board: &Board, from: Square, color: Color, dx: i8, dy: i8) -> Vec<Move> {
    let mut moves = Vec::with_capacity(7);

    let mut cur = from;

    loop {
        let Some(to) = cur.offset_if_valid(dx, dy) else {
            break;
        };

        match board.piece(to) {
            Some(piece) if piece.color == color => break, // Hit own piece
            Some(_) => {
                // Capture
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Capture,
                });
                break;
            }
            None => {
                // Quiet
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Quiet,
                });
                cur = to;
            }
        }
    }

    moves
}

fn pawn_pseudo_moves(board: &Board, from: Square, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(4);

    let dir: i8 = if color == Color::White { 1 } else { -1 };

    // Forward
    if let Some(single_step) = from.offset_if_valid(0, dir) {
        if board.piece(single_step).is_none() {
            moves.push(Move {
                from,
                to: single_step,
                kind: MoveKind::Quiet,
            });

            // Double step
            let rank = from.rank();
            let double_available =
                rank == 1 && color == Color::White || rank == 6 && color == Color::Black;

            if double_available {
                if let Some(double_step) = from.offset_if_valid(0, 2 * dir) {
                    if board.piece(double_step).is_none() {
                        moves.push(Move {
                            from,
                            to: double_step,
                            kind: MoveKind::DoublePawnPush,
                        });
                    }
                }
            }
        }
    }

    // Diagonal
    for dx in [-1i8, 1i8] {
        if let Some(diagonal_step) = from.offset_if_valid(dx, dir) {
            if let Some(other) = board.piece(diagonal_step) {
                if other.color != color {
                    moves.push(Move {
                        from,
                        to: diagonal_step,
                        kind: MoveKind::Capture,
                    });
                }
            }
        }
    }

    moves
}

impl Position {
    fn pseudo_legal_moves(&self) -> Vec<Move> {
        self.board
            .squares
            .iter()
            .enumerate()
            .flat_map(|(i, cell)| match cell {
                Some(piece) if piece.color == self.stm => {
                    let from = Square(i as u8);

                    match piece.kind {
                        Kind::Knight => {
                            leaper_pseudo_moves(&self.board, from, piece.color, &KNIGHT_OFFSETS)
                        }
                        Kind::Bishop => {
                            slider_psuedo_moves(&self.board, from, piece.color, &BISHOP_DIRS)
                        }
                        Kind::Rook => {
                            slider_psuedo_moves(&self.board, from, piece.color, &ROOK_DIRS)
                        }
                        Kind::Queen => {
                            slider_psuedo_moves(&self.board, from, piece.color, &QUEEN_DIRS)
                        }
                        Kind::King => {
                            leaper_pseudo_moves(&self.board, from, piece.color, &KNIGHT_OFFSETS)
                        }
                        Kind::Pawn => pawn_pseudo_moves(&self.board, from, piece.color),
                    }
                }
                _ => Vec::new(),
            })
            .collect()
    }
}

fn main() {
    let board = Board::start_pos();
    let castling = Castling {
        white: SideRights {
            king: true,
            queen: true,
        },
        black: SideRights {
            king: true,
            queen: true,
        },
    };

    let position = Position {
        board,
        stm: Color::White,
        castling: castling,
        ep: None,
        fullmove: 0,
        halfmove: 0,
    };

    let starting_moves = position.pseudo_legal_moves();
    println!("{}", &position.board);
    println!("{:?}", starting_moves);
    println!("{}", starting_moves.len());
}
