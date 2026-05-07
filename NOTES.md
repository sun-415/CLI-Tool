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
- Rust does not auto convert non-bool types to bool(unlike other coding lang where any non-zero value is true).
- Rust needs to know type of variables at compile time, so branches of if else statements need to match return type.

# Week 3 Notes
- This week I learned how structs and enums help model data more clearly in Rust.
- A struct is useful when multiple related pieces of data belong together. In this project, `Record` stores the three raw fields from one input line.
- An enum is useful when a value can be one of several fixed possibilities. For example, `Kind` can only be `Workout`, `Meal`, or `Sleep`.
- I also learned that enums are helpful for errors. `ParseError` lets me represent different failure cases like `EmptyLine`, `WrongFieldCount`, and `InvalidKind`.
- Compared to returning `bool`, returning `Result<Record, ParseError>` is more informative because it tells both whether parsing succeeded and why it failed.
- Even though it is confusing to learn in the beginning, `Option<Kind>` is useful when a value might or might not exist. `Some(...)` means valid kind, and `None` means invalid kind.
- `match` is very important when working with enums like `Option` and `Result` because it forces me to handle each possible case.
- This week helped me understand that Rust code can feel strict, but that makes the code clearer and safer.

# Week 4 Notes
- This week I refactored my project from one `main.rs` file into multiple modules: `lib.rs`, `model.rs`, `parse.rs`, `validate.rs`, and `report.rs`.
- I learned that `lib.rs` is the library crate root and defines the module tree, while `main.rs` should stay thin and mostly call into the library code.
- I start to understand the difference between `mod` and `use`. `mod` declares modules once, while `use` brings names into scope so we can use them in different files.
- Learning the module tree structure was new and a bit confusing at first. I had to understand that `lib.rs` builds the tree and other files just use it, instead of redefining modules everywhere.
- I added a new `Entry` struct to represent fully validated data. This helped me separate raw parsed input (`Record`) from validated data (`Entry`).
- I added `InvalidAmount` and `InvalidDate` to my error enum so the program can track more detailed rejection reasons.
- For amount validation, I used `amount_raw.parse::<u32>()` with `match`. I learned that parsing into `u32` automatically rejects negative numbers like `-5` and also non-integers like `abc`.
- For date validation, I implemented a basic format check for `YYYY-MM-DD` using string operations (length, dash positions, and digit checks). This is a simple placeholder and will be improved later with `chrono`.
- I used `BTreeMap<String, u64>` for tracking rejection reasons. I learned that `BTreeMap` keeps keys sorted, which makes the output deterministic.
- I practiced using `entry(...).or_insert(...)` again to count rejection reasons. At first I was confused about how it works, especially why it returns a mutable reference and why we need `*count += 1`.
- I also learned more about `String` vs `&str`, which is very different from other languages. This part was confusing:
  - `&str` is a reference and does not own data, while `String` owns data.
  - Some functions return `&str` (like `trim()`), while others return `String` (like `replace()` or `to_lowercase()`).
  - I cannot use indexing like `s[0]` in Rust because strings are UTF-8.
  - Instead, I have to use slicing like `&s[0..4]` or iterate with `.chars()`.

# Week 5 Notes
- This week I learned more about Rust error handling and how useful `Result<T, E>` and `Option<T>` really are in actual code.
- I was honestly surprised by how often `Result` and `Option` show up and how helpful they are once I started understanding the pattern more.
- At first they felt confusing and annoying because I kept having to unwrap, match, or convert values, but now I see that they make the program deal with errors and missing values in a very explicit way.
- I learned how the `?` operator makes code much shorter by returning early on errors instead of writing a lot of nested `match` code.
- I added `chrono` and used `NaiveDate::parse_from_str` for real calendar validation, which is much stronger than my Week 4 placeholder date check.
- I also learned that `chrono` alone checks whether a date is a real calendar date, but I still wanted to keep a strict format check for exact `YYYY-MM-DD`, so I combined both.
- This helped me understand the difference between malformed dates like `01-10-2026` and impossible dates like `2026-02-30`.
- I changed `Entry.date` from `String` to `NaiveDate`, which helped me understand the difference between raw text input and validated typed data.
- I practiced using `ok_or(...)` and `map_err(...)`. These were hard to understand at first because they convert `Option` into `Result` and also convert one error type into another.
- One thing I struggled with was understanding why a function can only return one error type in `Result<T, E>` and why different errors sometimes need to be converted or wrapped.
- I also learned to separate fatal errors from non-fatal errors. Invalid CLI usage and unreadable files should stop the whole program, but bad records inside the file should just be counted as rejected and allow the report to finish.
- Adding the `--strict` flag helped me understand that a CLI can still finish all processing and print the report, but then choose a non-zero exit code at the end if rejected records should count as failure.
- I also learned that usage messages and fatal errors should go to stderr, while normal report output should go to stdout.

# Week 6 Notes
- This week I added a `Processor` trait so valid `Entry` values can be handled by a replaceable processing step.
- The default implementation is `CountingProcessor`, which keeps the current behavior by returning a small report delta with one valid record.
- I made the pipeline explicit as `parse -> validate -> process -> report` inside the library-level `process_str` function.
- `main.rs` now still handles CLI arguments and file reading, but the record-processing logic lives in the library and is easier to test.
- I learned that a trait can describe behavior without forcing the rest of the program to know the concrete type doing the work.
- Passing `&mut impl Processor` lets the processor keep state later if needed, while still allowing simple code today.
- Returning a report delta from `process` made the trait match the curriculum wording without changing the output format.
- I added a unit test for `process_str`, which works with an input string and does not need file I/O.
