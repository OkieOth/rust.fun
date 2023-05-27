use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::any::type_name;
use serde_json;

use crate::serde_json::Value;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_file_as_string(file_name: &str) -> std::io::Result<()> {
    //let error_msg = format!("Can't read the desired file: {}", file_name);
    match fs::read_to_string(file_name) {
        Ok(s) => {
            //println!("file content: \n{s}");
            let jsonContent: HashMap<String, serde_json::Value> = serde_json::from_str(&s).expect("Error while deserialize JSON");
            for key in jsonContent.keys() {
                match jsonContent.get(key) {
                    Some(v) => {
                        let st :&str;
                        match v {
                            Value::Null => st = "Null",
                            Value::Bool(_) => st = "bool",
                            Value::Number(_) => st = "number",
                            Value::String(_) => st = "string",
                            Value::Array(_) => st = "array",
                            Value::Object(_) => st = "object",
                        }
                        print!("Key: {}, value type: {} -> {}\n", key, type_of(v), st);
                    },
                    None => print!("   can't find key: {}", key),
                }
            }
            return Ok(())
        },
        Err(e) => return Err(e),
    }
}

fn read_file_with_buffer(file_name: &str) -> std::io::Result<()> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(error) => return Err(error),
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}


fn main() ->  std::io::Result<()> {
    let file_name = "resources/yacg_model_schema.json";
    read_file_as_string(file_name).expect("error in read_file_as_string");
    //read_file_with_buffer(file_name).expect("error in read_file_with_buffer");
    Ok(())
}

