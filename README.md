# fake_serialize
A rust macro for implementing dummy implementations of the traits `serde::Serialize` and `serde::Deserialize`

Sometimes, you run in the problem to use some trait or function from an external crate, that is
constraint to types that implement the traits `serde::Serialize` or/and `serde::Deserialize`, even if 
this is not an requirement for all use cases. Here, using a dummy implementation that just returns
an error if `serialize` or `deserialize` is calls comes handy.

This crate provides a derive macro that just provides these dummy implementations, e.g.
```rust
use fake_serialize::{FakeSerialize,FakeDeserialize};

#[derive(FakeSerialize,FakeDeserialize)]
struct SomeStruct {
    ...
}
```

## License

Licensed under either of

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](https://github.com/argmin-rs/argmin/blob/master/LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
  * MIT License ([LICENSE-MIT](https://github.com/argmin-rs/argmin/blob/master/LICENSE-MIT) or
    http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

License: MIT OR Apache-2.0
