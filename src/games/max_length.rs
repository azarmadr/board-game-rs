use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::board::{Board, BoardAvailableMoves, Outcome, Player};

/// A wrapper around an existing board that has the same behaviour,
/// except that the outcome is a draw after a fixed number of moves has been played.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MaxMovesBoard<B: Board> {
    inner: B,
    moves: u64,
    max_moves: u64,
}

impl<B: Board> MaxMovesBoard<B> {
    pub fn new(inner: B, max_moves: u64) -> Self {
        MaxMovesBoard {
            inner,
            moves: 0,
            max_moves,
        }
    }

    pub fn inner(&self) -> &B {
        &self.inner
    }
}

impl<B: Board> Board for MaxMovesBoard<B> {
    type Move = B::Move;
    type Symmetry = B::Symmetry;

    fn can_lose_after_move() -> bool {
        B::can_lose_after_move()
    }

    fn next_player(&self) -> Player {
        self.inner.next_player()
    }

    fn is_available_move(&self, mv: Self::Move) -> bool {
        assert!(!self.is_done());
        self.inner.is_available_move(mv)
    }

    fn random_available_move(&self, rng: &mut impl Rng) -> Self::Move {
        assert!(!self.is_done());
        self.inner.random_available_move(rng)
    }

    fn play(&mut self, mv: Self::Move) {
        assert!(!self.is_done());
        self.inner.play(mv);
        self.moves += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.moves == self.max_moves {
            Some(Outcome::Draw)
        } else {
            self.inner.outcome()
        }
    }

    fn map(&self, sym: Self::Symmetry) -> Self {
        MaxMovesBoard {
            inner: self.inner.map(sym),
            moves: self.moves,
            max_moves: self.max_moves,
        }
    }

    fn map_move(sym: Self::Symmetry, mv: Self::Move) -> Self::Move {
        B::map_move(sym, mv)
    }
}

impl<'a, B: Board> BoardAvailableMoves<'a, MaxMovesBoard<B>> for MaxMovesBoard<B> {
    type AllMoveIterator = <B as BoardAvailableMoves<'a, B>>::AllMoveIterator;
    type MoveIterator = <B as BoardAvailableMoves<'a, B>>::MoveIterator;

    fn all_possible_moves() -> Self::AllMoveIterator {
        B::all_possible_moves()
    }

    fn available_moves(&'a self) -> Self::MoveIterator {
        assert!(!self.is_done());
        self.inner.available_moves()
    }
}

impl<B: Board> Display for MaxMovesBoard<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nmoves: {}/{:?}", self.inner, self.moves, self.max_moves)
    }
}