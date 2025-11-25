// 2025 Steven Chiacchira
use crate::matrix::{ToroidalBinaryMatrix, ToroidalMatrixIndex};
use std::fmt;
use std::mem;

/// The character used to represent an [`Automaton`]'s `true` state in files and `String`
/// representations.
const TRUE_CHAR: char = '#';
/// The character used to represent an [`Automaton`]'s `false` state in files and String
/// representations.
const FALSE_CHAR: char = '.';

#[derive(Clone, Debug)]
/// Defines how an [`Automaton`] will change from one state to the next.
///
/// This struct is limited to symmetrical cellular automaton rules defined over a (Moore Neighborhood)[https://en.wikipedia.org/wiki/Moore_neighborhood].
/// Roughly speaking, a cellular automaton rule is symmetric if it only considers the *number* of
/// alive and dead neighbors for a given cell.
pub struct AutomatonRule {
    /// A 9-element array of booleans. If the ith element is `true`, then a dead cell with `i`
    /// alive neighbors will become alive.
    ///
    /// # Examples
    /// The `born` array `[true, true, false, false, false, false, false, false, false]`
    /// specifies that only cells with 0 or 1 neighboring alive cells will become alive.
    pub born: [bool; 9],
    /// A 9-element array of booleans. If the ith element is `true`, then a living cell with `i`
    /// alive neighbors will die.
    ///
    /// # Examples
    /// The `dies` array `[true, true, false, false, false, false, false, false, false]`
    /// specifies that only cells with 0 or 1 neighboring alive cells will die.
    pub dies: [bool; 9],
}

#[derive(Debug)]
/// Defines a 2D, binary cellular automaton on a torus.
///
/// Uses a type `T` implementing `ToroidalBinaryMatrix` to store its state.
pub struct Automaton<T: ToroidalBinaryMatrix> {
    /// The automaton rule the Automaton will follow.
    rule: AutomatonRule,
    /// The initial state of the Automaton.
    state: T,
}

impl<T: ToroidalBinaryMatrix + Clone> Automaton<T> {
    /// Creates a new Automaton instance.
    ///
    /// # Arguments
    /// * `state` - the initial state of the Automaton. Implicitly defines the size of the automaton
    /// * `rule` - the rule the Automaton will use to generate its next state
    ///
    /// # Returns
    /// The created Automaton instance.
    pub fn new(state: T, rule: AutomatonRule) -> Self {
        Automaton { rule, state }
    }
    /// Iterates the Automaton's rule `iterations` times.
    ///
    /// # Arguments
    /// * `iterations` - the number of times to apply the Automaton's rule
    pub fn iter_rule(&mut self, iterations: u32) {
        let (rows, cols) = (self.state.get_rows(), self.state.get_cols());

        let mut copy = self.state.clone();
        for _ in 0..iterations {
            for row in 0..rows {
                for col in 0..cols {
                    let idx = (row as isize, col as isize);
                    let n_alive_neighbors = self.alive_neighbors(idx);

                    if self.state.at(&idx) {
                        copy.set(&idx, !self.rule.dies[n_alive_neighbors as usize]);
                    } else {
                        copy.set(&idx, self.rule.born[n_alive_neighbors as usize]);
                    }
                }
            }

            mem::swap(&mut copy, &mut self.state);
        }
    }

    /// Returns a reference to the Automaton state.
    ///
    /// # Returns
    /// The Automaton's internal state
    pub fn get_state(&self) -> &T {
        &self.state
    }

    /// Sets the state of one of the Automaton's cells.
    ///
    /// # Arguments
    /// * `idx` - the cell to be mutated
    /// * `value` - the state to set the cell to. `true` represents an alive cell, while `false`
    ///   represents a dead cell
    ///
    /// # Returns
    /// The original state of the mutated cell
    pub fn set_state(&mut self, idx: &ToroidalMatrixIndex, value: bool) -> bool {
        self.state.set(idx, value)
    }

    /// Counts the number of alive [Moore
    /// neighbors](https://en.wikipedia.org/wiki/Moore_neighborhood) at `idx`.
    ///
    /// Note that because the Moore neighborhood is a 3x3 area centered at `idx`, and because the
    /// cell at `idx` is not generally included in the count, the result will range in [0, 8].
    ///
    /// <div class="warning">
    /// The cell at `idx` will be included in the count if the width or height of the Automaton is 1.
    /// In this case, the result will still range in [0, 8].
    /// </div>
    ///
    /// # Arguments
    /// * `idx` - the cell to count living moore neighbors of.
    ///
    /// # Returns
    /// The number of living Moore neighbors of the cell at idx.
    pub fn alive_neighbors(&self, idx: ToroidalMatrixIndex) -> u32 {
        let (row, col) = (idx.0, idx.1);
        let mut sum_neighbors = 0;

        for r in (row - 1)..=(row + 1) {
            for c in (col - 1)..=(col + 1) {
                sum_neighbors += self.state.at(&(r, c)) as u32
            }
        }

        sum_neighbors -= self.state.at(&(row, col)) as u32;

        sum_neighbors
    }
}

impl<T: ToroidalBinaryMatrix + Clone> fmt::Display for Automaton<T> {
    /// Represents the state of the [`Automaton`] as a rectangular array of characters.
    /// # Example
    /// an Automaton with the state
    /// ```txt
    /// TFFT
    /// TFTT
    /// TTTT
    /// ```
    /// Will be represented as
    /// ```txt
    /// #..#
    /// #.##
    /// ####
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (rows, cols) = (self.state.get_rows(), self.state.get_cols());
        let mut result: String =
            String::with_capacity((self.state.get_rows() + 1) * self.state.get_cols());

        for row in 0..rows {
            let row_str = (0..cols)
                .map(|c| match self.state.at(&(row as isize, c as isize)) {
                    true => TRUE_CHAR,
                    false => FALSE_CHAR,
                })
                .collect::<String>();
            result.push_str(&row_str);
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}
