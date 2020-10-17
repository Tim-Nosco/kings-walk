# kings-walk

Example usage:
```rust
// Input n x n board (n=3):
// 0 0 1
// 0 2 0
// 9 0 0
let mut state = State::new([0,0,1,0,2,0,9,0,0],3);
state.hillclimb();
println!("{:?}", state);
// Outputs
// [4, 5, 1]
// [3, 2, 6]
// [9, 8, 7]
// score: 8
```
