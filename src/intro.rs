use serde::{Deserialize, Serialize};

// adding this to the custom data structure automatically makes
// them serializable and deserializable
#[derive(Serialize, Deserialize, Debug)]
pub struct Dog {
    // this basically translate to, convert name to pet_name during serialization
    // convert pet_name to name during deserialization
    #[serde(rename = "pet_name")]
    pub name: String,
    pub color: String,
    pub breed: String,
}

