extern crate itertools;
extern crate serde_json;
use itertools::Itertools;
use serde_json::Value;

fn main() {
    let s = ::std::fs::read_to_string("input").unwrap();
    let groups = s.bytes().group_by(|c| b"-1234567890".contains(c));
    let sum = groups
        .into_iter()
        .map(|(k, group)| {
            if k {
                let s = String::from_utf8(group.collect_vec()).unwrap();
                s.parse::<isize>().unwrap()
            } else {
                0
            }
        }).sum::<isize>();
    println!("sum: {}", sum);

    let v: Value = serde_json::from_str(&s).unwrap();
    println!("sum: {}", s1(&v));
    println!("sum: {}", s2(&v));
}

fn s1(v: &Value) -> isize {
    match v {
        &Value::Null => 0,
        &Value::Bool(_) => 0,
        &Value::Number(ref n) => n.as_i64().unwrap() as isize,
        &Value::String(_) => 0,
        &Value::Array(ref vec) => vec.iter().map(s1).sum::<isize>(),
        &Value::Object(ref map) => map.values().map(s1).sum::<isize>(),
    }
}

fn s2(v: &Value) -> isize {
    let red = Value::String("red".into());
    match v {
        &Value::Null => 0,
        &Value::Bool(_) => 0,
        &Value::Number(ref n) => n.as_i64().unwrap() as isize,
        &Value::String(_) => 0,
        &Value::Array(ref vec) => vec.iter().map(s2).sum::<isize>(),
        &Value::Object(ref map) => if map.values().any(|v| v == &red) {
            0
        } else {
            map.values().map(|v| s2(v)).sum::<isize>()
        },
    }
}
