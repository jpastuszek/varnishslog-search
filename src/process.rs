extern crate serde_json;
extern crate dot_json;

use serde_json::Value;
use std::io::{self, BufReader, BufRead};
use dot_json::value_to_dot;

fn main() {
    for line in BufReader::new(io::stdin()).lines() {
        let obj: Value = serde_json::from_str(&line.expect("failed to read input")).unwrap();
        let mut dot_obj = value_to_dot(&obj);

        for (_key, value) in dot_obj.as_object_mut().unwrap().iter_mut() {
            let to_str = match value {
                &mut Value::Number(ref num) if num.is_u64() => false,
                &mut Value::String(_) => false,
                _ => true
            };
            if to_str {
                *value = Value::String(value.to_string());
            }
        }

        dot_obj.as_object_mut().unwrap().insert("_doc".to_string(), Value::String(obj.to_string()));

        println!("{}", serde_json::to_string(&dot_obj).unwrap());
    }
}

