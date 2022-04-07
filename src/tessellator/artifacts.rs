use super::types::{GpuPrimitive, GpuTransform, GpuVertex};

#[derive(Debug)]
pub struct TessellationData {
    pub vertices: Vec<GpuVertex>,
    pub indices: Vec<u32>,
    pub transforms: Vec<GpuTransform>,
    pub primitives: Vec<GpuPrimitive>,
}
