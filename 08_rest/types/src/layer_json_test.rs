use uuid::Uuid;
use crate::layer;
use serde_json;

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

    println!("Serialized JSON:\n{}", json_string);

    // Deserialize the JSON string back to an object
    let deserialized_layer: layer::Layer =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");

    println!("Deserialized Object:\n{:?}", deserialized_layer);

}


