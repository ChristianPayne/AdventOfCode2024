# Advent Of Code 2024
# Workspaces
## Adding a new workspace
`cargo new --lib day-00` at the root of the repo and cargo will scaffold a new library.  
Add a new `bin` folder inside of the `src` folder and put the `part1.rs`, `part2.rs` and `input.txt` files in that.
## Running a day
Make sure to `cd` into the day you want to run.
### Testing sample data
Run `cargo test` to execute all of the tests in that workspace.  
`cargo test --bin part1` will run just a certain part's tests.  
Adding the `-- --nocapture` flags on tests will allow print and debug statements into the terminal.  
`cargo test --bin part1 -- --nocapture`  
`cargo test --bin part2 -- --nocapture`  
### Running real input data
Run `cargo run --bin part1` to run a specific bin. There is no "run all" command because we are not compiling binaries.  
`cargo run --bin part1`  
`cargo run --bin part2`  
