# Error handling in rust

Error handling in Rust with pattern matching

This small project exemplifies error handling in Rust using custom enums and pattern matching.

## Language characteristics

- `Struct`s
  are custom types, much like in C
- `Enum`s
  are types that can assume one of a few different variants, including optional arguments
- `Trait`s
  are collection of methods, similar to interfaces, that can be implemented to any data type
- No Exceptions
  Rust `Panic!` can be similar to exceptions, but they are unrecoverable.
  Recoverable errors are in general represented by Structs or Enums, usually wrapped in the `Result::Err` enum.
- `Result`
  is an Enum that can have either an `Ok` or an `Err` value
- No implicitly type conversion
  Conversion between two types must be implemented using the `From<>` trait.
- The `match` keyword
  is used for pattern matching and have a syntax similar to C Switch, while allowing destructuering

## References

- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
- https://doc.rust-lang.org/book/ch09-00-error-handling.html
- https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
- https://doc.rust-lang.org/rust-by-example/flow_control/match.html
- https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
