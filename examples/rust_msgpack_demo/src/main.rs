use rust_msgpack::decode;
use rust_msgpack::encode;
use std::collections::HashMap;
use value::from_value::FromValue;
use value::into_value::IntoValue;
use value_derive::*;

#[derive(Debug, Default, FromValue, IntoValue)]
struct Student {
    name: String,
    age: i32,
    sub: Sub,
}

#[derive(Debug, Default, FromValue, IntoValue)]
struct Sub {
    a: i32,
    b: bool,
    c: HashMap<String, String>,
}

fn main() {
    let mut s1 = Student {
        name: "huangjian".to_string(),
        age: 10000,
        sub: Sub {
            a: 100,
            ..Default::default()
        },
        ..Default::default()
    };
    s1.sub.c = HashMap::new();
    s1.sub.c.insert("language".to_string(), "Rust".to_string());

    println!("s1 = {:?}\n", s1);

    let bin = encode::encode(&s1).unwrap();
    println!("bin = {:?}\n", bin);

    let v2 = decode::decode_to_value(&bin).unwrap();
    println!("v2 = {}", v2);

    let s2: Student = v2.from_value();
    println!("s2 = {:?}", s2);
}
