// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: golang_types.mako v1.1.0)

use uuid::Uuid;
use std::collections::HashMap;
use crate::layer;




#[test]
fn test_layer() {
    let id = Uuid::nil();
    let name = "dummy".to_string();
    let l = layer::Layer::new(
        id,
        name
    );
}



#[test]
fn test_layer_content() {
    let id = Uuid::nil();
    let l = layer::LayerContent::new(
        id
    );
}


#[test]
fn test_geometry() {
    let l = layer::Geometry::new(
    );
}



#[test]
fn test_display_config() {
    let l = layer::DisplayConfig::new(
    );
}


#[test]
fn test_color() {
    let red = 42;
    let green = 42;
    let blue = 42;
    let l = layer::Color::new(
        red,
        green,
        blue
    );
}


#[test]
fn test_display_config_stroke() {
    let l = layer::DisplayConfigStroke::new(
    );
}


#[test]
fn test_display_config_fill() {
    let l = layer::DisplayConfigFill::new(
    );
}





