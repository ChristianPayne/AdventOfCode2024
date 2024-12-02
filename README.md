# Advent Of Code 2024
# Workspaces
## Adding a new workspace
`cargo new --lib day-00` at the root of the repo and cargo will scaffold a new library.  
Add a new `bin` folder inside of the `src` folder and put the `part1.rs`, `part2.rs` and `input.txt` files in that.
## Running a day
While inside of a day, run `cargo test` to execute all of the tests in that workspace.  
`cargo test --bin part1` will run just a certain part's tests.