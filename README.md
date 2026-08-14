# oo-rust

This repository demonstrates several object-oriented programming features available in Rust and explores whether they
make Rust an almost complete object-oriented language.

## Demonstrated features

The examples demonstrate the following OO-related features:
- Encapsulation
  - Grouping state and behavior in structs and `impl` blocks
  - Controlling access through Rust's module-based visibility system
- Trait implementation
  - Implementing shared behavior through the `Shape` trait
  - Polymorphism and dynamic dispatch through trait objects

Rust does not provide class inheritance. This repository explores an inheritance-like pattern using the `Deref` trait
and deref coercion. The pattern allows the concrete shape types to access shared `ShapeData` members and methods and
to be used where a reference to `ShapeData` is accepted.

## About the `Deref` pattern

Using `Deref` to emulate inheritance is controversial. The
[Rust Design Patterns guide](https://rust-unofficial.github.io/patterns/anti_patterns/deref.html) lists it as an
anti-pattern because it can make APIs less explicit and differs from conventional Rust design.

This repository uses the pattern as an educational experiment, not as a general-purpose recommendation. It offers
inheritance-like access to the embedded base data, but it does not introduce native class inheritance or formal
subtyping. Rust traits provide interface-like abstraction, while Rust privacy remains module-based rather than
class-based.

## Running the example

Run the example with:

```sh
cargo run
```

The program creates circles, rectangles, and squares; accesses their shared shape data; and processes them through a
`Vec<&dyn Shape>` to demonstrate dynamic dispatch.