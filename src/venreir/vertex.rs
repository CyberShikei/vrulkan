use std::mem::size_of;

use vulkanalia::{vk, vk::HasBuilder};

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: Vec2,
    color: Vec3,
}

impl Vertex {
    pub const fn new(pos: Vec2, color: Vec3) -> Self {
        Self { pos, color }
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)
            .build();

        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<Vec2>() as u32)
            .build();

        [pos, color]
    }
}
