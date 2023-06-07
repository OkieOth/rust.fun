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

    id: Uuid,

    name: String,

    min_zoom: i32,

    max_zoom: i32,

    description: String,

    // this attrib has no real value, it's only there to challenge the enum handling
    dummy: Option<LayerDummyEnum>,
}


pub enum LayerDummyEnum {
    value1,
    value2,
    value3,
}



/* a feature to display
*/
pub struct LayerContent {

    id: Uuid,

    layer_id: Uuid,

    projection: String,

    geometry: Option<Geometry>,

    // container for additional key/value pairs
    data: HashMap<String, String>,

    display: Option<DisplayConfig>,
}



/* Geometry definition to display the feature
*/
pub struct Geometry {

    point: Vec<f32>,

    // a geometry consisting of multiple separate points
    multi_point: Vec<Vec<f32>>,

    // a geometry consisting of multiple connected line segments
    line_string: Vec<Vec<f32>>,

    // a geometry consisting of multiple multi-lines
    multi_line_string: Vec<Vec<Vec<f32>>>,

    // a closed geometry consisting of multiple connected line segments
    polygon: Vec<Vec<Vec<f32>>>,

    // a geometry consisting of multiple separate polygons
    multi_polygon: Vec<Vec<Vec<Vec<f32>>>>,
}





/* Optional configuration to display a feature
*/
pub struct DisplayConfig {

    stroke: Option<DisplayConfigStroke>,

    fill: Option<DisplayConfigFill>,

    icon: String,
}



/* The color definition to display a feature
*/
pub struct Color {

    red: i32,

    green: i32,

    blue: i32,

    alpha: i32,
}



pub struct DisplayConfigStroke {

    width: i32,

    dash_array: Vec<i32>,

    dash_offset: i32,

    color: Option<Color>,
}



pub struct DisplayConfigFill {

    color: Option<Color>,
}










