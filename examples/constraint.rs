/// Calling a generic functions which constraint to trait `Serialize`
/// even for types than can not implement a proper `Serialize` trait

use fake_serialize::{FakeSerialize,FakeDeserialize};
use std::fmt::Debug;

#[cfg(feature="json")]
use serde_json;
use serde::Serialize;

#[cfg(not(feature="json"))]
fn main() {
    println!("To run this example, please use --features json");
}


fn custom_print<T>(t: &T, as_json: bool) 
where T: Debug + Serialize,
{
    if as_json {
        let json = serde_json::to_string(t).unwrap();
        println!("{:?}", json)
    } else {
        println!{"{:?}", *t}
    }
}


/// Struct with serialize/deserialize disabled
#[derive(Debug,FakeSerialize,FakeDeserialize)]
struct NotSerializable {
    i: i16,
}

#[cfg(feature="json")]
fn main() {
    let not_serializable = NotSerializable{ i: 1 };
    println!("Succussful call to function with Serialize constraint, as long as serialization is not used:");
    custom_print(&not_serializable, false);
}

