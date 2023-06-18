// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: rust_types_json_test.mako v1.1.0)

use uuid::Uuid;
use std::collections::HashMap;
use crate::layer;
use serde_json;




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
fn test_json_geometry() {
    let first = layer::Geometry::new(
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::Geometry =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
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
fn test_json_display_config_stroke() {
    let first = layer::DisplayConfigStroke::new(
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: layer::DisplayConfigStroke =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
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





