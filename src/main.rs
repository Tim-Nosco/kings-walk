#![allow(unused_mut, unused_variables, dead_code)]

use rand::Rng;

// Holds the filled out game board which is a [1,n*n] permutation and
// a vec of indicies which to the board that are mutable.
#[derive(Debug)]
pub struct State {
	board: Vec<u8>,
	n: usize,
	assignments: Vec<usize>,
	rng: rand::rngs::ThreadRng,
}

impl State {
	// Create a new state object
	fn new(board: Vec<u8>, n: usize) -> State {
		let mut state = State {
			board: board,
			n: n,
			assignments: Vec::new(),
			rng: rand::thread_rng(),
		};
		// Identify the mutable positions of the board and determine
		// what values are taken.
		let mut seen = vec![false; n * n + 1];
		for (idx, &start_value) in state.board.iter().enumerate() {
			if start_value == 0 {
				// Save the position index
				state.assignments.push(idx);
			}
			// Record the seen value
			seen[start_value as usize] = true;
		}
		// Assign the remaining values by overwriting the zeros
		let mut next_unseen = 1;
		for idx in &state.assignments {
			// Increment the next_unseen pointer until it points to
			// something that is unassigned
			while seen[next_unseen] {
				next_unseen += 1;
			}
			// Assign this position on the board to the unusued value
			state.board[*idx] = next_unseen as u8;
			// Ensure to move to the next position.
			next_unseen += 1;
		}
		state
	}
	// Swap assignments to create a new random start
	fn random_start(&mut self) {
		// Create a sequence of (idx1, idx2) so that idx1 can be
		// swapped with idx2. idx1 will be the position in the array
		// and idx2 will be the random value stored at that location.
		let swaps: Vec<usize> = (1..self.assignments.len())
			.rev()
			.map(|x| self.rng.gen::<usize>() % x)
			.collect();
		for (idx1, &idx2) in swaps.iter().enumerate() {
			self.board.swap(
				self.assignments[idx1],
				self.assignments[idx2 + idx1],
			);
		}
	}
	// Score the board in its current state
	fn score(&self) -> usize {
		#[inline]
		// helper function to determine sum of neighbors' scores
		fn helper(goal: u8, neighbors: &[u8]) -> usize {
			// the edge cases of 0 and n*n+1 don't matter unless
			// the board uses all 255 values that an u8 can represent
			let goal1 = goal + 1;
			let goal2 = goal - 1;
			neighbors
				.iter()
				.map(
					|&x| if x == goal1 || x == goal2 { 1 } else { 0 },
				)
				.sum()
		}
		// Go through every position on the board to determine
		// it's score
		let mut total = 0;
		for (idx, &goal) in self.board.iter().enumerate() {
			// of all 8 neighbors of each position on the board,
			// only consider the one to the right and the 3 below
			// because as we progress through the board, the positions
			// behind us have already been considered
			let mut neighbors = vec![idx + self.n];
			// if youre left aligned, skip bottom left
			if idx % self.n != 0 {
				// bottom left
				neighbors.push(idx + self.n - 1);
			}
			// if youre right alined, skip right and bottom right
			if idx % self.n != self.n - 1 {
				// to the right
				neighbors.push(idx + 1);
				// bottom right
				neighbors.push(idx + self.n + 1);
			}

			let mut valid_neighbors: Vec<u8> = Vec::new();
			for neighbor in neighbors {
				// If that neighbor position is still on the board,
				// save it
				if let Some(&v) = self.board.get(neighbor) {
					valid_neighbors.push(v);
				}
			}
			// add 0, 1, or 2 depending on how many neighbors were
			// correctly assigned
			total += helper(goal, &valid_neighbors);
		}
		total
	}
	// The max score is the size of number of edges (verticies - 1)
	#[inline]
	fn max_score(&self) -> usize {
		self.board.len() - 1
	}
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;

	#[test]
	fn score_should_work2() {
		// Make a new state
		let mut state =
			State::new(vec![0, 0, 1, 0, 2, 0, 9, 0, 0], 3);
		// ensure the board is properly setup
		state.board = vec![3, 4, 1, 8, 2, 5, 9, 7, 6];
		// 3 4 1
		// 8 2 5
		// 9 7 6
		// total: 8
		assert_eq!(state.score(), state.max_score());
	}
	#[test]
	fn score_should_work1() {
		// Make a new state
		let state = State::new(vec![0, 0, 1, 0, 2, 0, 9, 0, 0], 3);
		// ensure the board is properly setup
		assert_eq!(vec![3, 4, 1, 5, 2, 6, 9, 7, 8], state.board);
		// 3 4 1	1-2-3-4-5 	score: 4
		// 5 2 6	6-7-8		score: 2
		// 9 7 8	9			score: 0
		// 						total: 6
		assert_eq!(state.score(), 6);
	}
	#[test]
	fn random_start_should_only_have_unique_values() {
		// Make a new state
		let mut state =
			State::new(vec![0, 0, 1, 0, 2, 0, 9, 0, 0], 3);
		// Randomize it
		state.random_start();
		// ensure each value is unique and in the range [1,n*n]
		let mut seen = HashSet::new();
		for x in &state.board {
			assert!(!seen.contains(x));
			assert!(*x > 0 && *x <= 9);
			seen.insert(*x);
		}
		println!("{:?}", state);
	}
	#[test]
	fn new_states_should_only_have_unique_values() {
		// Make a new state
		let state = State::new(vec![0, 0, 1, 0, 2, 0, 9, 0, 0], 3);
		// ensure the board is properly setup
		assert_eq!(vec![3, 4, 1, 5, 2, 6, 9, 7, 8], state.board);
	}
}
