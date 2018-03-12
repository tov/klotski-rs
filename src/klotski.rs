//! An instance of the `Puzzle` trait for Klotski.

use super::generic_solver::*;

use self::Piece::*;
use self::Direction::*;

const WIDTH: usize = 4;
const HEIGHT: usize = 5;

const N_PIECES: usize = 10;
const N_DIRECTIONS: usize = 4;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
#[repr(i8)]
enum Piece {
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    H0 = 4,
    S0 = 5,
    V0 = 6,
    V1 = 7,
    V2 = 8,
    V3 = 9,
    X0 = -1,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
#[repr(i8)]
enum Direction {
    North = 0 * N_PIECES as i8,
    South = 1 * N_PIECES as i8,
    West  = 2 * N_PIECES as i8,
    East  = 3 * N_PIECES as i8,
}

const PIECES: [Piece; N_PIECES] =
    [C0, C1, C2, C3, H0, S0, V0, V1, V2, V3];

const DIRECTIONS: [Direction; N_DIRECTIONS] =
    [North, South, West, East];

impl Direction {
    fn from(self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            North => if y > 0 {Some((x, y - 1))} else {None},
            South => if y < HEIGHT - 1 {Some((x, y + 1))} else {None},
            West => if x > 0 {Some((x - 1, y))} else {None},
            East => if x < WIDTH - 1 {Some((x + 1, y))} else {None},
        }
    }
}

/// A Klotski move.
#[derive(Eq, Debug, PartialEq, Copy, Clone, Hash)]
pub struct Move {
    piece: Piece,
    direction: Direction,
}

impl Move {
    fn to_u64(self) -> u64 {
        (self.piece as i8 + self.direction as i8) as u64
    }

    fn from_u64(n: u64) -> Self {
        Move {
            piece: PIECES[n as usize % N_PIECES],
            direction: DIRECTIONS[n as usize / N_PIECES],
        }
    }
}

// A `MoveSet` is a bitset of moves, where 0 means allowed and 1 means disallowed.
/// A set of Klotski moves, which can be iterated over.
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct MoveSet(u64);

impl MoveSet {
    fn new() -> Self {
        MoveSet(0)
    }

    fn remove(&mut self, a_move: Move) {
        self.0 |= 1u64 << a_move.to_u64();
    }

    fn is_allowed(&self, a_move: Move) -> bool {
        self.0 & (1u64 << a_move.to_u64()) == 0
    }
}

/// An iterator over a set of Klotski moves.
pub struct MoveSetIter {
    set: MoveSet,
    next: u64,
}

impl Iterator for MoveSetIter {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        while (self.next as usize) < N_PIECES * N_DIRECTIONS {
            let the_move = Move::from_u64(self.next);
            self.next += 1;
            if self.set.is_allowed(the_move) {
                return Some(the_move);
            }
        }

        None
    }
}

impl IntoIterator for MoveSet {
    type Item = Move;
    type IntoIter = MoveSetIter;

    fn into_iter(self) -> Self::IntoIter {
        MoveSetIter { set: self, next: 0, }
    }
}

/// The state of the Klotski board.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Klotski([BoardRow; HEIGHT]);

type BoardRow = [Piece; WIDTH];

const INITIAL_BOARD: Klotski =
    Klotski([[V0, S0, S0, V1],
             [V0, S0, S0, V1],
             [V2, H0, H0, V3],
             [V2, C0, C1, V3],
             [C2, X0, X0, C3]]);

impl Klotski {
    /// Returns the initial board.
    pub fn initial() -> Self {
        INITIAL_BOARD
    }
}

impl Puzzle for Klotski {
    type Move = Move;
    type MoveSet = MoveSet;

    fn is_final(&self) -> bool {
        self.0[3][1] == S0
            && self.0[3][2] == S0
            && self.0[4][1] == S0
            && self.0[4][2] == S0
    }

    fn get_possible_moves(&self) -> MoveSet {
        let mut result = MoveSet::new();

        for (y, row) in self.0.iter().enumerate() {
            for (x, &piece) in row.iter().enumerate() {
                if piece == X0 {continue;}
                for &direction in DIRECTIONS.iter() {
                    if let Some((nx, ny)) = direction.from(x, y) {
                        let target = self.0[ny][nx];
                        if !(target == X0 || target == piece) {
                            result.remove(Move { piece, direction });
                        }
                    } else {
                        result.remove(Move { piece, direction });
                    }
                }
            }
        }

        result
    }

    fn make_move(&self, a_move: Move) -> Self {
        let mut result = Klotski([[X0; WIDTH]; HEIGHT]);

        for (y, row) in self.0.iter().enumerate() {
            for (x, &piece) in row.iter().enumerate() {
                if piece == X0 {continue;}
                if piece == a_move.piece {
                    let (nx, ny) = a_move.direction.from(x, y).expect("apply_move");
                    result.0[ny][nx] = piece;
                } else {
                    result.0[y][x] = piece;
                }
            }
        }

        result
    }
}

