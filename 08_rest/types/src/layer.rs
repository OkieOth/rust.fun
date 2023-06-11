// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: rust_types.mako v0.1.0)

use serde_json;
use serde::Deserialize;
use uuid::Uuid;
use std::collections::HashMap;

/* A layer definition
*/
pub struct Layer {

    pub id: Uuid,

    pub name: String,

    pub min_zoom: Option<i32>,

    pub max_zoom: Option<i32>,

    pub description: Option<String>,

    // this attrib has no real value, it's only there to challenge the enum handling
    pub dummy: Option<LayerDummyEnum>,
}

impl Layer {
    pub fn new (
        id: Uuid,
        name: String,
    ) -> Self {
        Self {
            id: id,
            name: name,
            min_zoom: None,
            max_zoom: None,
            description: None,
            dummy: None,
        }
    }
}


pub enum LayerDummyEnum {
    Value1,
    Value2,
    Value3,
}

impl LayerDummyEnum {
    fn as_str(&self) -> &'static str {
        match *self {
            LayerDummyEnum::Value1 => "value1",
            LayerDummyEnum::Value2 => "value2",
            LayerDummyEnum::Value3 => "value3",
        }
    }

    fn from_str<'a>(s:&'a str) -> Result<LayerDummyEnum, &'static str> {
        match s {
            "value1" => Ok(LayerDummyEnum::Value1),
            "value2" => Ok(LayerDummyEnum::Value2),
            "value3" => Ok(LayerDummyEnum::Value3),
            _ => Err("unknown value"),
        }
    }

}


/* a feature to display
*/
pub struct LayerContent {

    pub id: Uuid,

    pub layer_id: Option<Uuid>,

    pub projection: Option<String>,

    pub geometry: Option<Geometry>,

    // container for additional key/value pairs
    pub data: Option<HashMap<String, String>>,

    pub display: Option<DisplayConfig>,
}

impl LayerContent {
    pub fn new (
        id: Uuid,
    ) -> Self {
        Self {
            id: id,
            layer_id: None,
            projection: None,
            geometry: None,
            data: None,
            display: None,
        }
    }
}


/* Geometry definition to display the feature
*/
pub struct Geometry {

    pub point: Option<Vec<f32>>,

    // a geometry consisting of multiple separate points
    pub multi_point: Option<Vec<Vec<f32>>>,

    // a geometry consisting of multiple connected line segments
    pub line_string: Option<Vec<Vec<f32>>>,

    // a geometry consisting of multiple multi-lines
    pub multi_line_string: Option<Vec<Vec<Vec<f32>>>>,

    // a closed geometry consisting of multiple connected line segments
    pub polygon: Option<Vec<Vec<Vec<f32>>>>,

    // a geometry consisting of multiple separate polygons
    pub multi_polygon: Option<Vec<Vec<Vec<Vec<f32>>>>>,
}

impl Geometry {
    pub fn new (
    ) -> Self {
        Self {
            point: None,
            multi_point: None,
            line_string: None,
            multi_line_string: None,
            polygon: None,
            multi_polygon: None,
        }
    }
}



/* Optional configuration to display a feature
*/
pub struct DisplayConfig {

    pub stroke: Option<DisplayConfigStroke>,

    pub fill: Option<DisplayConfigFill>,

    pub icon: Option<String>,
}

impl DisplayConfig {
    pub fn new (
    ) -> Self {
        Self {
            stroke: None,
            fill: None,
            icon: None,
        }
    }
}


/* The color definition to display a feature
*/
pub struct Color {

    pub red: i32,

    pub green: i32,

    pub blue: i32,

    pub alpha: Option<i32>,
}

impl Color {
    pub fn new (
        red: i32,
        green: i32,
        blue: i32,
    ) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
            alpha: None,
        }
    }
}


pub struct DisplayConfigStroke {

    pub width: Option<i32>,

    pub dash_array: Option<Vec<i32>>,

    pub dash_offset: Option<i32>,

    pub color: Option<Color>,
}

impl DisplayConfigStroke {
    pub fn new (
    ) -> Self {
        Self {
            width: None,
            dash_array: None,
            dash_offset: None,
            color: None,
        }
    }
}


pub struct DisplayConfigFill {

    pub color: Option<Color>,
}

impl DisplayConfigFill {
    pub fn new (
    ) -> Self {
        Self {
            color: None,
        }
    }
}







#[cfg(test)]
#[path = "./layer_test.rs"]
mod layer_test;
