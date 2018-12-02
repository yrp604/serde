#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
#[serde(untagged)]
#[serde(tag = "t", content = "c")]
enum E {
    A(u8),
    B(String),
}

fn main() {}
