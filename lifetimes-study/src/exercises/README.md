# Lifetimes, borrowing & ownership — exercises

Ten exercises to practice **lifetimes**, **borrowing**, and **ownership** in Rust. Each file has a short concept explanation, function stubs (with `todo!()`), and tests.

## How to use

- **Implement** the functions: replace `todo!("...")` with your code.
- **Run tests** for one exercise: `cargo test exercise_01` (or `exercise_02`, etc.)
- **Run all exercise tests**: `cargo test exercises::`

## Exercise list

| # | File | Concept |
|---|------|--------|
| 01 | `exercise_01_longest` | Basic lifetime annotation on a function that returns a reference |
| 02 | `exercise_02_struct_holding_ref` | Struct that holds a reference; lifetime on the struct |
| 03 | `exercise_03_multiple_lifetimes` | Two lifetime parameters; return type tied to one of them |
| 04 | `exercise_04_elision` | When to add explicit lifetimes (elision not enough) |
| 05 | `exercise_05_borrow_checker` | Avoiding conflicting borrows (immutable vs mutable) |
| 06 | `exercise_06_impl_lifetime` | Lifetime in `impl` block and methods returning references |
| 07 | `exercise_07_return_ref_from_struct` | Method returning a reference that comes from the struct |
| 08 | `exercise_08_mutable_exclusive` | Exclusive mutable borrow; swapping without overlapping borrows |
| 09 | `exercise_09_slice_lifetimes` | Owned `String` vs `&str`; return slice tied to `&self` |
| 10 | `exercise_10_option_ref` | `Option<&'a T>` and lifetime in return type |

## Tips

- If the compiler asks for a lifetime, add the minimal one that makes the rule “returned reference must not outlive the data it points into” hold.
- For mutable borrows, avoid holding two `&mut` to the same data; use indices or `split_first_mut` / `split_last_mut` to get non-overlapping parts.
- Run `cargo test` often; when all tests pass, move on to the next exercise.
