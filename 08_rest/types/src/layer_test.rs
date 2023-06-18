// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: rust_types_test.mako v1.1.0)

use uuid::Uuid;
use std::collections::HashMap;
use crate::layer;




#[test]
fn test_layer() {
    let id = Uuid::nil();
    let name = "dummy".to_string();
    let _l = layer::Layer::new(
        id,
        name
    );
}



#[test]
fn test_layer_content() {
    let id = Uuid::nil();
    let _l = layer::LayerContent::new(
        id
    );
}


#[test]
fn test_geometry() {
    let _l = layer::Geometry::new(
    );
}



#[test]
fn test_display_config() {
    let _l = layer::DisplayConfig::new(
    );
}


#[test]
fn test_color() {
    let red = 42;
    let green = 42;
    let blue = 42;
    let _l = layer::Color::new(
        red,
        green,
        blue
    );
}


#[test]
fn test_display_config_stroke() {
    let _l = layer::DisplayConfigStroke::new(
    );
}


#[test]
fn test_display_config_fill() {
    let _l = layer::DisplayConfigFill::new(
    );
}





