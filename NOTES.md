# Week 1 Notes
- I learned how to create a Rust binary crate using `cargo new`.
- For macros like `println!(This is {}, variable)`, `{}` is a placeholder and rust fills palceholders left to right so the first `{}` gets the first argument after the string.
- `std::env::args()` lets me read command-line arguments.
- `std::fs::read_to_string` to read an entire file into a String.
- Rust puts the program name in args[0], so the first real argument is args[1].
- I handled errors using `match` so the program exits with a non-zero code on failure.
- `.lines().count()` is an easy way to count how many lines are in a file.
