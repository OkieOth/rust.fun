// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: rust_types.mako v0.1.0)

use serde_json;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

/* A layer definition
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {

    #[serde(rename = "id")]
    pub id: Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "minZoom")]
    pub min_zoom: Option<i32>,

    #[serde(rename = "maxZoom")]
    pub max_zoom: Option<i32>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    // this attrib has no real value, it's only there to challenge the enum handling
    #[serde(rename = "dummy")]
    pub dummy: Option<LayerDummyEnum>,
}

impl PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool {
        if self.id != other.id {
            return false;
        }
        if self.name != other.name {
            return false;
        }
        if self.min_zoom != other.min_zoom {
            return false;
        }
        if self.max_zoom != other.max_zoom {
            return false;
        }
        if self.description != other.description {
            return false;
        }
        if self.dummy != other.dummy {
            return false;
        }
        return true;
    }
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


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum LayerDummyEnum {
    #[serde(rename = "value1")]
    Value1,
    #[serde(rename = "value2")]
    Value2,
    #[serde(rename = "value3")]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerContent {

    #[serde(rename = "id")]
    pub id: Uuid,

    #[serde(rename = "layerId")]
    pub layer_id: Option<Uuid>,

    #[serde(rename = "projection")]
    pub projection: Option<String>,

    #[serde(rename = "geometry")]
    pub geometry: Option<Geometry>,

    // container for additional key/value pairs
    #[serde(rename = "data")]
    pub data: Option<HashMap<String, String>>,

    #[serde(rename = "display")]
    pub display: Option<DisplayConfig>,
}

impl PartialEq for LayerContent {
    fn eq(&self, other: &Self) -> bool {
        if self.id != other.id {
            return false;
        }
        if self.layer_id != other.layer_id {
            return false;
        }
        if self.projection != other.projection {
            return false;
        }
        if self.geometry != other.geometry {
            return false;
        }
        if self.data != other.data {
            return false;
        }
        if self.display != other.display {
            return false;
        }
        return true;
    }
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {

    #[serde(rename = "point")]
    pub point: Option<Vec<f32>>,

    // a geometry consisting of multiple separate points
    #[serde(rename = "multiPoint")]
    pub multi_point: Option<Vec<Vec<f32>>>,

    // a geometry consisting of multiple connected line segments
    #[serde(rename = "lineString")]
    pub line_string: Option<Vec<Vec<f32>>>,

    // a geometry consisting of multiple multi-lines
    #[serde(rename = "multiLineString")]
    pub multi_line_string: Option<Vec<Vec<Vec<f32>>>>,

    // a closed geometry consisting of multiple connected line segments
    #[serde(rename = "polygon")]
    pub polygon: Option<Vec<Vec<Vec<f32>>>>,

    // a geometry consisting of multiple separate polygons
    #[serde(rename = "multiPolygon")]
    pub multi_polygon: Option<Vec<Vec<Vec<Vec<f32>>>>>,
}

impl PartialEq for Geometry {
    fn eq(&self, other: &Self) -> bool {
        if self.point != other.point {
            return false;
        }
        if self.multi_point != other.multi_point {
            return false;
        }
        if self.line_string != other.line_string {
            return false;
        }
        if self.multi_line_string != other.multi_line_string {
            return false;
        }
        if self.polygon != other.polygon {
            return false;
        }
        if self.multi_polygon != other.multi_polygon {
            return false;
        }
        return true;
    }
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
#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayConfig {

    #[serde(rename = "stroke")]
    pub stroke: Option<DisplayConfigStroke>,

    #[serde(rename = "fill")]
    pub fill: Option<DisplayConfigFill>,

    #[serde(rename = "icon")]
    pub icon: Option<String>,
}

impl PartialEq for DisplayConfig {
    fn eq(&self, other: &Self) -> bool {
        if self.stroke != other.stroke {
            return false;
        }
        if self.fill != other.fill {
            return false;
        }
        if self.icon != other.icon {
            return false;
        }
        return true;
    }
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Color {

    #[serde(rename = "red")]
    pub red: i32,

    #[serde(rename = "green")]
    pub green: i32,

    #[serde(rename = "blue")]
    pub blue: i32,

    #[serde(rename = "alpha")]
    pub alpha: Option<i32>,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        if self.red != other.red {
            return false;
        }
        if self.green != other.green {
            return false;
        }
        if self.blue != other.blue {
            return false;
        }
        if self.alpha != other.alpha {
            return false;
        }
        return true;
    }
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


#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayConfigStroke {

    #[serde(rename = "width")]
    pub width: Option<i32>,

    #[serde(rename = "dashArray")]
    pub dash_array: Option<Vec<i32>>,

    #[serde(rename = "dashOffset")]
    pub dash_offset: Option<i32>,

    #[serde(rename = "color")]
    pub color: Option<Color>,
}

impl PartialEq for DisplayConfigStroke {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width {
            return false;
        }
        if self.dash_array != other.dash_array {
            return false;
        }
        if self.dash_offset != other.dash_offset {
            return false;
        }
        if self.color != other.color {
            return false;
        }
        return true;
    }
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


#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayConfigFill {

    #[serde(rename = "color")]
    pub color: Option<Color>,
}

impl PartialEq for DisplayConfigFill {
    fn eq(&self, other: &Self) -> bool {
        if self.color != other.color {
            return false;
        }
        return true;
    }
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

#[cfg(test)]
#[path = "./layer_json_test.rs"]
mod layer_json_test;

#[cfg(test)]
#[path = "./layer_manual_tests.rs"]
mod layer_manual_tests;
