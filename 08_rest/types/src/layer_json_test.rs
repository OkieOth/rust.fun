// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: rust_types_json_test.mako v1.1.0)

use uuid::Uuid;
use std::collections::HashMap;
use crate::layer;
use serde_json;
use std::fs;
use std::env;
use std::path::Path;




#[test]
fn test_json_layer() {
    let id = Uuid::nil();
    let name = "dummy".to_string();
    let first = layer::Layer::new(
        id,
        name
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::Layer =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}

#[test]
fn test_file_load_Layer() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomObj/Layer.json",
        false => "resources/randomObj/Layer.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: layer::Layer = serde_json::from_str(string.as_str()).unwrap();
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}

#[test]
fn test_file_load2_Layer() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomLists/Layer.json",
        false => "resources/randomLists/Layer.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: Vec<layer::Layer> = serde_json::from_str(string.as_str()).unwrap();
            assert!(l.len()>2);
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}



#[test]
fn test_json_layer_content() {
    let id = Uuid::nil();
    let first = layer::LayerContent::new(
        id
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::LayerContent =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}

#[test]
fn test_file_load_LayerContent() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomObj/LayerContent.json",
        false => "resources/randomObj/LayerContent.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: layer::LayerContent = serde_json::from_str(string.as_str()).unwrap();
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}

#[test]
fn test_file_load2_LayerContent() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomLists/LayerContent.json",
        false => "resources/randomLists/LayerContent.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: Vec<layer::LayerContent> = serde_json::from_str(string.as_str()).unwrap();
            assert!(l.len()>2);
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}


#[test]
fn test_json_geometry() {
    let first = layer::Geometry::new(
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::Geometry =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}

#[test]
fn test_file_load_Geometry() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomObj/Geometry.json",
        false => "resources/randomObj/Geometry.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: layer::Geometry = serde_json::from_str(string.as_str()).unwrap();
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}

#[test]
fn test_file_load2_Geometry() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomLists/Geometry.json",
        false => "resources/randomLists/Geometry.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: Vec<layer::Geometry> = serde_json::from_str(string.as_str()).unwrap();
            assert!(l.len()>2);
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}



#[test]
fn test_json_display_config() {
    let first = layer::DisplayConfig::new(
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::DisplayConfig =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}

#[test]
fn test_file_load_DisplayConfig() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomObj/DisplayConfig.json",
        false => "resources/randomObj/DisplayConfig.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: layer::DisplayConfig = serde_json::from_str(string.as_str()).unwrap();
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}

#[test]
fn test_file_load2_DisplayConfig() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomLists/DisplayConfig.json",
        false => "resources/randomLists/DisplayConfig.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: Vec<layer::DisplayConfig> = serde_json::from_str(string.as_str()).unwrap();
            assert!(l.len()>2);
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}


#[test]
fn test_json_color() {
    let red = 42;
    let green = 42;
    let blue = 42;
    let first = layer::Color::new(
        red,
        green,
        blue
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::Color =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}

#[test]
fn test_file_load_Color() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomObj/Color.json",
        false => "resources/randomObj/Color.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: layer::Color = serde_json::from_str(string.as_str()).unwrap();
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}

#[test]
fn test_file_load2_Color() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomLists/Color.json",
        false => "resources/randomLists/Color.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: Vec<layer::Color> = serde_json::from_str(string.as_str()).unwrap();
            assert!(l.len()>2);
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}


#[test]
fn test_json_display_config_stroke() {
    let first = layer::DisplayConfigStroke::new(
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::DisplayConfigStroke =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}

#[test]
fn test_file_load_DisplayConfigStroke() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomObj/DisplayConfigStroke.json",
        false => "resources/randomObj/DisplayConfigStroke.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: layer::DisplayConfigStroke = serde_json::from_str(string.as_str()).unwrap();
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}

#[test]
fn test_file_load2_DisplayConfigStroke() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomLists/DisplayConfigStroke.json",
        false => "resources/randomLists/DisplayConfigStroke.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: Vec<layer::DisplayConfigStroke> = serde_json::from_str(string.as_str()).unwrap();
            assert!(l.len()>2);
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}


#[test]
fn test_json_display_config_fill() {
    let first = layer::DisplayConfigFill::new(
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::DisplayConfigFill =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}

#[test]
fn test_file_load_DisplayConfigFill() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomObj/DisplayConfigFill.json",
        false => "resources/randomObj/DisplayConfigFill.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: layer::DisplayConfigFill = serde_json::from_str(string.as_str()).unwrap();
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}

#[test]
fn test_file_load2_DisplayConfigFill() {
    let file_name = match Path::new("./08_rest").is_dir() {
        true => "./08_rest/types/resources/randomLists/DisplayConfigFill.json",
        false => "resources/randomLists/DisplayConfigFill.json",
    }; 

    assert!(Path::new(file_name).is_file(), "can't find json file to load");
    match fs::read_to_string(file_name) {
        Ok(string) => {
            let l: Vec<layer::DisplayConfigFill> = serde_json::from_str(string.as_str()).unwrap();
            assert!(l.len()>2);
        },
        Err(e) => {
            let error_msg = e.to_string();
            println!("Error: {}", error_msg);
            println!("Error while reading file: {}", file_name);
            assert!(false, "Error while reading file");
        },
    }
}





