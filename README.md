# sdi

Rust statically resolved dependency injection library

## Usage

### provide!

Register an statically resolved service expression indexed by key.

```rust
use sdi::{inject, provide};

#[derive(Debug, PartialEq)]
struct A;

impl A {
    pub fn new() -> A { A }
    provide!(A <- A::new());
}

assert_eq!(A::new(), inject!(A))
```
Provide by inject is also ok.
```rust
use sdi::{inject, provide};

#[derive(Debug, PartialEq)]
struct A;
provide!(A <- A);

#[derive(Debug, PartialEq)]
struct B(A);

impl B {
    pub fn new(a:A) -> B { B(a) }
    provide!(B <- B::new(inject!(A)));
}

assert_eq!(B::new(A), inject!(B))
```

### inject!

Get an statically resolved service expression by key.

```rust
use sdi::{inject, provide};

#[derive(Debug, PartialEq)]
struct A;

impl A {
    pub fn new() -> A { A }
    provide!(A <- A::new());
}

assert_eq!(A::new(), inject!(A))
```
