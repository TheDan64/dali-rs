use crate::stipple::Stipple;
use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum StippleSemantics {
    // reference vertex positions with the co variable in vertex shaders
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,

    // reference vertex instance’s position on screen
    #[sem(
        name = "translation",
        repr = "[f32; 2]",
        wrapper = "VertexInstanceTranslation"
    )]
    InstanceTranslation,

    // reference vertex size in vertex shaders (used for vertex instancing)
    #[sem(name = "scale", repr = "[f32; 2]", wrapper = "VertexInstanceScale")]
    InstanceScale,

    #[sem(
        name = "colormap_scale",
        repr = "[f32; 2]",
        wrapper = "VertexInstanceColormapScale"
    )]
    InstanceColormapScale,

    #[sem(name = "rotation", repr = "f32", wrapper = "VertexInstanceRotation")]
    InstanceRotation,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics", instanced = "true")]
pub struct Instance {
    pub translation: VertexInstanceTranslation,
    pub scale: VertexInstanceScale,
    pub colormap_scale: VertexInstanceColormapScale,
    pub rotation: VertexInstanceRotation,
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Vertex {
        Vertex {
            position: VertexPosition::new(position),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics", instanced = "true")]
pub(crate) struct VertexInstance {
    pub translation: VertexInstanceTranslation,
    pub scale: VertexInstanceScale,
    pub colormap_scale: VertexInstanceColormapScale,
    pub rotation: VertexInstanceRotation,
}

impl From<&Stipple> for VertexInstance {
    fn from(stipple: &Stipple) -> Self {
        VertexInstance {
            translation: VertexInstanceTranslation::new(stipple.translation),
            scale: VertexInstanceScale::new(stipple.scale),
            colormap_scale: VertexInstanceColormapScale::new(stipple.colormap_scale),
            rotation: VertexInstanceRotation::new(stipple.rotation),
        }
    }
}
