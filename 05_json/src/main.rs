use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::any::type_name;
use serde_json;
use serde::Deserialize;
use uuid::Uuid;

use crate::serde_json::Value;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug, Deserialize)]
struct SimpleJsonCont {
    #[serde(rename = "b")]
    b_value: Option<bool>,
    #[serde(rename = "string")]
    string_value: Option<String>,
    #[serde(rename = "zahl")]
    zahl_value: Option<f32>,
    #[serde(rename = "uuid")]
    uuid_value: Option<Uuid>,
}

fn read_file_as_string(file_name: &str) -> std::io::Result<()> {
    //let error_msg = format!("Can't read the desired file: {}", file_name);
    match fs::read_to_string(file_name) {
        Ok(s) => {
            //println!("file content: \n{s}");
            let json_content: HashMap<String, serde_json::Value> = serde_json::from_str(&s).expect("Error while deserialize JSON");
            for key in json_content.keys() {
                match json_content.get(key) {
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
        Err(e) => {
            println!("Error while reading file: {}", file_name);
            return Err(e)
        },
    }
}

fn read_file_with_buffer(file_name: &str) -> std::io::Result<()> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(error) => return Err(error),
    };
    let reader = BufReader::new(file);
    let mut s: String = String::from("");
    for line in reader.lines() {
        let line = line?;
        //println!("{}", line);
        s = format!("{}{}", s, line);
    }
    let json_cont: Vec<SimpleJsonCont> = serde_json::from_str(&s).unwrap();
    print!("I deserialized {} elems.\n", json_cont.len());
    let mut i: u8 = 0;
    for j in json_cont {
        print!("Object #{}: \n", i);
        if j.b_value.is_some() {
            print!("    b_value: {}\n", j.b_value.unwrap());
        }
        if j.string_value.is_some() {
            print!("    string_value: {}\n", j.string_value.unwrap());
        }
        if j.zahl_value.is_some() {
            print!("    zahl_value: {}\n", j.zahl_value.unwrap());
        }
        if j.uuid_value.is_some() {
            print!("    uuid_value: {}\n", j.uuid_value.unwrap());
        }
        print!("\n");
        i = i+1;
    }
    Ok(())
}


fn main() ->  std::io::Result<()> {
    let file_name = "resources/yacg_model_schema.json";
    read_file_as_string(file_name).expect("error in read_file_as_string");
    let file_name2 = "resources/simple.json";
    read_file_with_buffer(file_name2).expect("error in read_file_with_buffer");
    let file_name3 = "resources/simple2.json";
    read_file_with_buffer(file_name3).expect("error in read_file_with_buffer");
    let file_name4 = "resources/simple2222.json";
    match read_file_as_string(file_name4) {
        Ok(_) => println!("Would have expect an error in reading not existing files :-/"),
        Err(_) => println!("Got expected error :)"),
    }
    Ok(())
}

