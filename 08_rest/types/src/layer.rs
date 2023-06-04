// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: rust_types.mako v0.1.0)



/* A layer definition
*/
pub struct Layer {

    Id uuid.UUID

    Name string

    MinZoom optional.Optional[int32]

    MaxZoom optional.Optional[int32]

    Description optional.Optional[string]

    // this attrib has no real value, it's only there to challenge the enum handling
    Dummy OptionalLayerDummyEnum
}






/* a feature to display
*/
pub struct LayerContent {

    Id uuid.UUID

    LayerId optional.Optional[uuid.UUID]

    Projection optional.Optional[string]

    Geometry OptionalGeometry

    // container for additional key/value pairs
    Data optional.Optional[map[string]string]

    Display OptionalDisplayConfig
}



/* Geometry definition to display the feature
*/
pub struct Geometry {

    Point []float64

    // a geometry consisting of multiple separate points
    MultiPoint [][]float64

    // a geometry consisting of multiple connected line segments
    LineString [][]float64

    // a geometry consisting of multiple multi-lines
    MultiLineString [][][]float64

    // a closed geometry consisting of multiple connected line segments
    Polygon [][][]float64

    // a geometry consisting of multiple separate polygons
    MultiPolygon [][][][]float64
}





/* Optional configuration to display a feature
*/
pub struct DisplayConfig {

    Stroke OptionalDisplayConfigStroke

    Fill OptionalDisplayConfigFill

    Icon optional.Optional[string]
}



/* The color definition to display a feature
*/
pub struct Color {

    Red int32

    Green int32

    Blue int32

    Alpha optional.Optional[int32]
}



pub struct DisplayConfigStroke {

    Width optional.Optional[int32]

    DashArray []int32

    DashOffset optional.Optional[int32]

    Color OptionalColor
}



pub struct DisplayConfigFill {

    Color OptionalColor
}










