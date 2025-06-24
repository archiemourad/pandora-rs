use bytemuck::{Pod, Zeroable};

pub trait VertexLayout: Pod + Zeroable {
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}
