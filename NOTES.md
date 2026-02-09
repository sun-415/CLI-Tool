# Week 1 Notes
- I learned how to create a Rust binary crate using `cargo new`.
- For macros like `println!(This is {}, variable)`, `{}` is a placeholder and rust fills palceholders left to right so the first `{}` gets the first argument after the string.
- `std::env::args()` lets me read command-line arguments.
- `std::fs::read_to_string` to read an entire file into a String.
- Rust puts the program name in args[0], so the first real argument is args[1].
- I handled errors using `match` so the program exits with a non-zero code on failure.
- `.lines().count()` is an easy way to count how many lines are in a file.

# Week 2 Notes
- This week I struggled and learned the difference between ownership vs borrowing.
- To be more specific, the difference between `mut vec: Vec<T>` vs `vec: &mut Vec<T>`:
  - `mut vec: Vec<T>` means the function takes ownership (moves the vector in). `mut` only allows modifying the local binding. This could mean the original owner will no longer be valid.
  - `vec: &mut Vec<T>` means the function borrows the caller’s vector mutably and can modify the original without taking ownership.
- The difference between `mut` and `&mut`:
  - `mut` controls whether a variable binding can be changed.
  - `&mut` is a mutable borrow (exclusive access) to someone else’s data.
- Ownership vs borrowing:
  - Passing `Vec<T>` moves ownership, so the original variable becomes invalid after the call.
  - Passing `&T` or `&mut T` borrows temporarily, so the original variable stays usable after the borrow ends. 
  - `&T`is for many immutable borrows whereas `&mut T` is exactly one mutable borrow.
- Because this content is more new to me, I find that it is easier to read the chapter material first rather than jumping straight into rustlings exercises.
