use uuid::Uuid;
use crate::layer;
use serde_json;
use std::fs;

#[test]
fn test_dummy() {
    let layer = layer::Layer {
        id: Uuid::new_v4(),
        name: String::from("Layer Name"),
        min_zoom: Some(5),
        max_zoom: Some(10),
        description: Some(String::from("Layer Description")),
        dummy: Some(layer::LayerDummyEnum::Value2),
    };

    // Serialize the object to a JSON string
    let json_string = serde_json::to_string(&layer).expect("Failed to serialize object to JSON");

    // Deserialize the JSON string back to an object
    let mut deserialized_layer: layer::Layer =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");

    assert_eq!(layer, deserialized_layer);

    deserialized_layer.name = "schnulli".to_string();

    assert_ne!(layer, deserialized_layer);
}

#[test]
fn test_file_load() {
    let file_name = "resources/randomObj/Layer.json";
    match fs::read_to_string(file_name) {
        Ok(string) => {
            //println!("file content: \n{s}");
            let l: layer::Layer = serde_json::from_str(string.as_str()).unwrap();
            assert_eq!(l.id, Uuid::parse_str("e535ba09-59f7-4d89-940f-f3b9fb628daa").unwrap());
            assert_eq!(l.name, "Sign chance try image no article fund care.".to_string());
            match(l.min_zoom) {
                Some(v) => assert_eq!(v, 11),
                None => assert!(false),
            };
            match(l.max_zoom) {
                Some(v) => assert_eq!(v, 20),
                None => assert!(false),
            };
            match(l.description) {
                Some(v) => assert_eq!(v, "I dog mouth ever know surface hold produce.".to_string()),
                None => assert!(false),
            };
            match(l.dummy) {
                Some(v) => assert_eq!(v.as_str(), "value3"),
                None => assert!(false),
            };

        },
        Err(e) => {
            println!("Error while reading file: {}", file_name);
            assert!(false);
        },
    }
}

