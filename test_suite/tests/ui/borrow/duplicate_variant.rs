#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Str<'a>(&'a str);

#[derive(Deserialize)]
enum Test<'a> {
    #[serde(borrow)]
    S(#[serde(borrow)] Str<'a>),
}

fn main() {}
