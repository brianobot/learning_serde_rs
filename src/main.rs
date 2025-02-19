#![allow(unused_imports, dead_code)]

pub mod intro;
pub use intro::*;

// use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::Serialize; // when using this particular Serialize you have to declare feature = ['derive'] in the project's toml file
// else use the serde_derive crate Serialize trait directly
// use serde_derive::Serialize;
use serde_json::to_string as to_json;
use serde_json::Error;
use learning_serde::hex_dump;
use serde_json::{to_string, from_str};


#[derive(Serialize)] // this here allows the Struct that this declaration applied on it
// to be become Serializable automatically, this is done by implementing
// the neccesary Traits (Serialize in this case) on the struct at Compile time, we can also activate that 
// serialization feature by directly implementing those traits ourselves instead of deriving it here.
// this provides the tooling for external formats to interact with rust code
struct Point {
    x: f64,
    y: f64,
}


fn main() {
    // Serde stands for Serialization and Deserialization
    // it abstract away the way we reason about serialization and deserializatio of data
    // serde library has been around for a long time and so it has it limitation and flaws
    // but it can do quite alot more

    // let point = Point { x: 1.0, y: 1.0 };

    // // now to convert our point to json we do this
    // let as_json = to_json(&point).unwrap();
    // // serdes understands many more formats than json

    // println!("json: {}", &as_json);

    // let _hex_dump_data = hex_dump();

    let bruno = Dog { 
        name: String::from("Bruno"), 
        color: String::from("Brown"), 
        breed: String::from("Ekuke") ,
    };

    // Serialization to JSON String
    let dog_ser = to_string(&bruno);
    // converts a serializable object/data structure to a json string
    let mut result: String = String::new();

    match dog_ser {
        Ok(value) => {
            println!("{:?}", value);
            result = value;
        },
        Err(msg) => println!("Error Occurred: {:?}", msg),
    }

    // Deserialization
    let dog_de: Result<Dog, Error> = from_str(&result);
    match dog_de {
        Ok(value) => println!("{:?}", value),
        Err(msg) => println!("Error Occurred: {:?}", msg),
    }
}
