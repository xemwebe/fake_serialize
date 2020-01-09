/// Calling serialize and deserialize for type 
/// that can not implement a proper `Serialize` trait

use fake_serialize::{FakeSerialize,FakeDeserialize};

#[cfg(feature="json")]
use serde_json;
use serde::{Deserialize, Serialize,};

#[cfg(not(feature="json"))]
fn main() {
    println!("To run this example, please use --features json");
}

/// Serializable struct
#[derive(Debug,Serialize,Deserialize)]
struct Serializable {
    i: i16,
}

/// Struct with serialize/deserialize disabled
#[derive(Debug,FakeSerialize,FakeDeserialize)]
struct NotSerializable {
    i: i16,
}

/// Struct which is serializable, but not deserializable
#[derive(Debug,Serialize,FakeDeserialize)]
struct HalfThisAndThat {
    i: i16,
}


#[cfg(feature="json")]
fn main() {
    let serializable = Serializable{ i: 1 };
    let serializable = serde_json::to_string(&serializable).unwrap();
    // // this will panic
    // let noser: NotSerializable = serde_json::from_str(&serializable).unwrap();
    // catch error
    let not_serializable: Result<NotSerializable, serde_json::Error> = serde_json::from_str(&serializable);
    println!("Deserialization with fake deserializer fails as expected: {:?}", not_serializable);

    let not_serializable = NotSerializable{ i: 2 };
    let not_serializable = serde_json::to_string(&not_serializable);
    println!("Serialization with fake serializer fails also as expected: {:?}", not_serializable);

    let half_this_and_that = HalfThisAndThat{ i: 3 };
    let half_this_and_that = serde_json::to_string(&half_this_and_that).unwrap();
    println!("HalfThisAndThat can be serialized: {:?}", half_this_and_that);
    let half_this_and_that: Result<HalfThisAndThat, serde_json::Error> = serde_json::from_str(&half_this_and_that);
    println!("But HalfThisAndThat cant' be deserialized: {:?}", half_this_and_that);
}
