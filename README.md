# derive-vec

This is a WIP, not many methods have been implemented thus far. 
PRs welcome.

[![CI](https://github.com/milesgranger/derive-vec/workflows/CI/badge.svg?branch=master)](https://github.com/milesgranger/derive-vec/actions?query=branch=master)

---

Derive `Vec` like behavior for a struct with an inner `Vec`.

Example
---

```rust
use derive_vec::{VecBehavior, VecTrait};

#[derive(VecBehavior, Default)]
struct Foo {
    #[vec]
    pub values: Vec<usize>,
}

fn main() {
    let mut foo = Foo::default();
    foo.push(1);
    assert_eq!(foo.values.len(), 1);
}
```

Status:
---
- [ ] Support for non-generic structs 
- [ ] Support for generic structs