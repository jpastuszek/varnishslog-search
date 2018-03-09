#[macro_use]
extern crate serde_json;
extern crate dot_json;

use serde_json::Value;
use std::io::{self, BufReader, BufRead};
use dot_json::value_to_dot;
use std::collections::HashMap;

#[derive(Debug)]
enum SchemaType {
    Text(Value),
    U64
}

fn main() {
    let mut schema = HashMap::new();

    for line in BufReader::new(io::stdin()).lines() {
        let obj: Value = serde_json::from_str(&line.expect("failed to read input")).unwrap();
        let dot_obj = value_to_dot(&obj);

        match dot_obj {
            Value::Object(map) => {
                for (key, value) in map.into_iter() {
                    let schema_type = match value {
                        Value::Number(ref num) if num.is_u64() => SchemaType::U64,
                        value => SchemaType::Text(value)
                    };

                    let insert = match (schema.get(&key), &schema_type) {
                        (Some(&SchemaType::Text(_)), &SchemaType::U64) => false, // keep Text over I64
                        _ => true
                    };
if insert {
                        schema.insert(key, schema_type);
                    }
                }
            }
            _ => panic!("not object!")
        }
    }

    let meta = json!({
        "segments": [],
        "schema": 
            schema.into_iter().map(|(key, schema_type)| 
                json!({
                    "name": key,
                    "type": match schema_type {
                        SchemaType::U64 => "u64",
                        SchemaType::Text(_) => "text"
                    },
                    "options": {
                        "indexing": {
                            "record": "position",
                            "tokenizer": "en_stem"
                        },
                        "indexed": true,
                        "stored": true
                    }
                })
            ).chain(Some(
                json!({
                    "name": "_doc",
                    "type": "text",
                    "options": {
                        "indexing": {
                            "record": "position",
                            "tokenizer": "en_stem"
                        },
                        "stored": true
                    }
                })
            )).collect::<Vec<_>>()
        ,
        "opstamp": 0 
    });
    println!("{}", serde_json::to_string_pretty(&meta).unwrap());
}
