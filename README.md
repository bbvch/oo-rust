# Modelling inheritance in Rust

```
cargo run     # all seven demos
cargo test    # 21 tests, one per claim made below
```

## First: decompose what you are actually asking for

`class Derived : public Base` in OOP is four separate features fused into one
keyword. Rust unbundles them, which is why there is no single answer — you pick
the mechanisms for the parts you need:

| OOP gives you                                                | you need it for | Rust mechanism |
|--------------------------------------------------------------|---|---|
| **State reuse** — derived gets base's members                | avoiding copy-pasted fields | a field + an accessor trait |
| **Implementation reuse** — derived gets base's method bodies | avoiding copy-pasted logic | trait provided methods, blanket impls, delegation |
| **Subtype polymorphism** — `Base*` points at a `Derived`     | heterogeneous collections | trait objects (`dyn Trait`) |
| **Open extensibility** — third parties add derived classes   | plugins, library boundaries | traits (open) vs. enums (closed) |

`Deref` fakes the first two by making `&Square` implicitly *become* `&ShapeData`,
and gives you nothing for the other two. That is why it's commonly called an anti-pattern: it
pays with implicitness for the two things that are cheapest to write explicitly,
while leaving the hard parts unsolved.

Concretely, what `Deref`-as-inheritance costs you:

- **Invisible method resolution.** `square.area()` might be `Square::area` or
  `Rectangle::area` depending on which inherent methods exist. Adding a method to
  the "base" can silently change what the "derived" call site does — and adding
  one to the derived type silently shadows the base's. Neither is a compile error.
- **No subtyping — at all.** `Deref` buys you method-call syntax and nothing
  else. If `Rectangle: Shape` and `Square: Deref<Target = Rectangle>`, then
  `square.area()` compiles, but *both* of these are hard errors:

  ```
  fn generic(t: &impl Shape)  -> f64 { t.area() }   // E0277: Square: Shape not satisfied
  fn object (t: &dyn Shape)   -> f64 { t.area() }   // E0277: same
  ```

  So you end up writing the trait impl anyway — at which point `Deref` was
  carrying no weight. (Verified against rustc 1.99; deref coercion resolves
  method calls, it does not participate in trait-bound resolution.)
- **Two impls for mutation.** `Deref` + `DerefMut`, and `DerefMut` on a
  non-pointer type is where the surprises get sharp.
- **It lies to the reader.** `Deref` means "this is a pointer to that". A
  `Square` is not a pointer to a `Rectangle`.

## The seven patterns

| # | Module | State reuse | Impl reuse | Dynamic dispatch | Open to new types | Boilerplate | Runtime cost |
|---|---|---|---|---|---|---|---|
| 1 | `p1_composition` | field | manual forwarding | no | yes | **high** (types × methods) | none |
| 2 | `p2_accessor_trait` | accessor trait | **provided methods** | yes (`dyn Shape`) | yes | low (2 methods/type) | vtable if `dyn` |
| 3 | `p3_supertraits` | via accessor | supertrait bounds | yes, + up/downcast | yes | low | vtable |
| 4 | `p4_enum` | plain field | one `match` | no (static `match`) | **no — closed** | **lowest** | branch, no vtable |
| 5 | `p5_data_oriented` | component table | systems (free fns) | no | yes (new tables) | medium | **best layout** |
| 6 | `p6_blanket_and_mixins` | via accessor | **blanket impl** (zero per-type) | optional | yes | **near zero** | none (monomorphized) |
| 7 | `p7_delegation_macro` | derived | derived from a field | yes | yes | near zero | none |

### 1. Explicit composition + manual delegation

The literal translation. `Square` holds a `Rectangle`, forwards by hand. Verbose,
but every inherited method is visible in the source and nothing resolves behind
your back. Two levels of "inheritance" cost nothing extra.

Use it when: there are two or three forwarded methods and you want zero magic.

### 2. Accessor trait + provided methods — **the default answer**

```rust
trait HasShapeData {                     // the protected members
    fn data(&self) -> &ShapeData;
    fn data_mut(&mut self) -> &mut ShapeData;
}

trait Shape: HasShapeData {
    fn area(&self) -> f64;               // pure virtual

    fn translate(&mut self, dx: f64, dy: f64) {   // non-virtual base method
        self.data_mut().translate(dx, dy);
    }

    fn describe(&self) -> String {        // template method / NVI:
        format!("{} covers {:.2} units^2", // base skeleton calling a virtual
                self.data().label(), self.area())
    }
}
```

This is the pattern that covers most of what people reach for `Deref` to get.
Every implementor writes exactly two trivial methods and inherits everything
else. Provided methods can be overridden, and the override can call the
"parent" implementation (`Square::describe` does). It works with both static
dispatch (`fn f<S: Shape>`) and `Box<dyn Shape>`.

The one boilerplate line per type is `impl HasShapeData`, and a five-line
`macro_rules!` erases even that.

### 3. Supertraits, upcasting, downcasting

`trait Drawable: Shape` is pure interface inheritance. Since Rust 1.86, trait
upcasting is stable, so `&dyn Drawable` coerces to `&dyn Shape` with no
hand-written `as_shape()` helper — the last real ergonomic gap in this area is
closed. `dyn Any` + `downcast_ref` covers `dynamic_cast`, with the same
"you may have a design problem" smell.

Use it for interface hierarchies. Note it inherits *requirements*, not state —
combine with pattern 2 for the data.

### 4. Struct + enum: a closed hierarchy

```rust
struct Shape { data: ShapeData, kind: ShapeKind }   // shared state, flat
enum ShapeKind { Circle { radius: f64 }, Rectangle { .. }, Square { .. } }
```

Shared state is a plain field — no trait needed at all. The virtual method is one
exhaustive `match`. The moment the set of subtypes is closed, this is
dramatically the simplest option, and it is the first genuinely *data-driven*
step: the subtype is a **value**, so it can be serialised, stored in config,
compared, and matched exhaustively — adding a variant makes the compiler point at
every site that must handle it.

Trade-offs, honestly: it is closed (third parties cannot add a shape), the enum
pads to its largest variant (`size_of::<Shape>() == 72` here), and adding a
*method* means touching one `match` rather than N files — the expression problem,
traded in the other direction from inheritance.

### 5. Data-oriented: entities, components, systems

Stop asking "how do I inherit?" and ask "what does this entity *have*?".

| OOP                    | here |
|------------------------|---|
| base class members     | a component table every entity has |
| derived class members  | a component table only some entities have |
| virtual call in a loop | one tight loop per table, no dispatch |
| multiple inheritance   | an entity registered in several tables |
| `dynamic_cast`         | a table lookup returning `Option` |

The `Square` question dissolves entirely: a square is not a type, it is a
rectangle whose two size components happen to be equal — so the classic
"is `Square` a subtype of `Rectangle`?" Liskov argument has nothing to attach to.

The payoff is memory layout, not syntax. `translate_all` sweeps one contiguous
`Vec<Point>` and touches no other bytes; the equivalent virtual call over
`Vec<Box<dyn Shape>>` chases a pointer and a vtable per element.

The version in `p5_data_oriented.rs` is hand-rolled for legibility, and its
component lookup is a linear scan. A production ECS (`bevy_ecs`, `hecs`,
`flecs`) replaces that with sparse sets or archetypes and adds queries,
scheduling, and change detection. Use one of those rather than growing this.

Use it when: you have many entities, you iterate them in bulk, and performance or
composability matters more than "one object, one type". Do **not** use it for
three shapes in a dialog box.

### 6. Blanket impls + generic mixins

```rust
impl<T: HasShapeData> Positioned for T { /* ... */ }
```

Nobody implements `Positioned`. Everyone gets it. This is implementation
inheritance granted by **capability rather than lineage** — and it applies
retroactively to types that gain the bound later, which no class hierarchy can
do. C++ has no direct equivalent; it is the closest thing Rust has to "add a
method to every subclass of `Base`".

`Bordered<S: Shape>` and `Scaled<S: Shape>` are the other half: mixins that add
state and behaviour to any shape and remain shapes themselves. `Scaled<Bordered<Circle>>`
stacks like a class hierarchy, except the layering order is chosen at the *use*
site and each layer is independently testable. This is the niche CRTP occupies in
C++, without the CRTP.

### 7. Macro-generated delegation

```rust
#[derive(Delegate)]
#[delegate(Shape, target = "rect")]     // reads like `: public Rectangle`
struct Square { rect: Rectangle }
```

The objection to `Deref` was never "forwarding is wrong", it was "forwarding
should be visible". `ambassador` puts it at the type declaration — exactly where
a C++ reader looks for the base clause — and generates a real trait impl, so
trait objects and generic bounds work. `delegate!` does the same for inherent
methods and can rename or post-process, which `Deref` cannot express at all.

Cost: two proc-macro dependencies and their compile time.

## Recommendation

**Start with pattern 2.** Accessor trait for the shared state, provided methods
for the shared behaviour, template method where base logic calls into derived
logic. It is the direct, honest translation of a C++ base class, it composes with
trait objects and generics, and it costs each type two trivial methods.

Then adjust:

- Subtype set closed and known? → **pattern 4**, and skip the traits entirely.
  Simplest thing that works.
- Many entities iterated in bulk, or "composition of capabilities" beats "one
  type per thing"? → **pattern 5**, using a real ECS crate.
- Forwarding boilerplate becoming the bulk of the file? → **pattern 7**.
- Behaviour that should apply to a whole *category* of types? → **pattern 6**'s
  blanket impl.
- Interface layering with `dyn` upcasting? → **pattern 3**.

And the framing worth taking away: the question "how do I do inheritance in
Rust?" usually resolves into "which of the four things was I using inheritance
for?" — after which the answer is normally a trait with default methods, or a
plain enum, and the `Deref` trick was never needed.
