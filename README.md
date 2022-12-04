# aoc2022

## Project structure
One Crate.   
Each day is a separate binary/library pair:
```
# bin
src/bin/dayXX.rs

# lib
src/dayXX.rs
# opt
src/dayXX/some_lib.rs
# Need to add each new library to lib.rs
src/lib.rs

# inputs (part 1, part 2)
inputs/dayXX_1.txt
inputs/dayXX_2.txt
```

## Run binary
```bash
cargo run --bin day03_bin
```

## Run tests
Pattern matching; I've prefixed each library's test function.
Only run library tests (code included in `./src/lib.rs`) and 
match test function names.   
```
cargo test --lib day04
cargo test --lib day04_1
```

