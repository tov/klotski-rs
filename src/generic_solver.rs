use std::collections::BTreeSet;
use std::collections::HashSet;
use std::hash::Hash;
use std::mem;

/// A simple interface for sets
pub trait Set<A: Eq> {
    /// Creates a new, empty set.
    fn new() -> Self;
    /// Adds an element to the set.
    fn add(&mut self, element: A);
    /// Checks for the presence of an element.
    fn mem(&self, candidate: &A) -> bool;
}

impl<A: Eq + Hash> Set<A> for HashSet<A> {
    fn new() -> Self {
        HashSet::new()
    }

    fn add(&mut self, element: A) {
        self.insert(element);
    }

    fn mem(&self, candidate: &A) -> bool {
        self.contains(candidate)
    }
}

impl<A: Eq + Ord> Set<A> for BTreeSet<A> {
    fn new() -> Self {
        BTreeSet::new()
    }

    fn add(&mut self, element: A) {
        self.insert(element);
    }

    fn mem(&self, candidate: &A) -> bool {
        self.contains(candidate)
    }
}

/// An interface to puzzla configurations.
pub trait Puzzle: Clone + Eq {
    type Move;
    type MoveIter: IntoIterator<Item = Self::Move>;
    fn make_move(&self, a_move: Self::Move) -> Self;
    fn get_possible_moves(&self) -> Self::MoveIter;
    fn is_final(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Path<P>(Vec<P>);

impl<P> Path<P> {
    fn new(start: P) -> Self {
        Path(vec![start])
    }

    fn last(&self) -> &P {
        self.0.last().expect("Path should be non-empty")
    }

    fn push(&mut self, step: P) {
        self.0.push(step);
    }

    pub fn into_vec(self) -> Vec<P> {
        self.0
    }
}

pub struct Solver<P: Puzzle, S: Set<P>> {
    seen: S,
    todo: Vec<Path<P>>,
}

impl<P: Puzzle, S: Set<P>> Solver<P, S> {
    pub fn new(initial_configuration: P) -> Self {
        Solver {
            seen: S::new(),
            todo: vec![Path::new(initial_configuration)],
        }
    }

    pub fn solve(mut self) -> Option<Vec<P>> {
        while !self.todo.is_empty() {
            let paths = mem::replace(&mut self.todo, Vec::new());
            for path in paths {
                if path.last().is_final() {
                    return Some(path.into_vec());
                }

                for each_move in path.last().get_possible_moves() {
                    let next_config = path.last().make_move(each_move);
                    if !self.seen.mem(&next_config) {
                        self.seen.add(next_config.clone());
                        let mut next_path = path.clone();
                        next_path.push(next_config);
                        self.todo.push(next_path);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Puzzle for i32 {
        type Move = i32;
        type MoveIter = Vec<i32>;

        fn make_move(&self, a_move: Self::Move) -> Self {
            *self + a_move
        }

        fn get_possible_moves(&self) -> Self::MoveIter {
            vec![-1, 2]
        }

        fn is_final(&self) -> bool {
            *self == 10
        }
    }

    fn solve_i32_game() -> Option<Vec<i32>> {
        let config = Solver::<i32, HashSet<i32>>::new(1);
        config.solve()
    }

    #[test]
    fn i32_game_test() {
        assert_eq!(solve_i32_game(), Some(vec![1, 0, 2, 4, 6, 8, 10]));
    }
}
