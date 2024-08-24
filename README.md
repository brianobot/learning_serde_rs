# Learning Serde in Rust

## Introduction
The serde crates serializes and deserializers Rust types to and from other formats

TL:DR;
- The serializer is passed to the serialize method of the Serialize trait implementation, 
- serializer does not have to produce bytes


## Note
It is important to note the declaration used in the project's Cargo.toml file, which specifies that the serde library is to be used and that it derive feature should be activated.

