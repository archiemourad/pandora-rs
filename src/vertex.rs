use bytemuck::{Pod, Zeroable};

pub trait Vertex: Pod + Zeroable {
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}
