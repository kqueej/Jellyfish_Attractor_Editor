use jellyfish::graphics::vertex::Vertex;

#[test]
fn vertex_size() {
    assert_eq!(std::mem::size_of::<Vertex>(), 16);
}

#[test]
fn vertex_bytes() {
    let vertices = [
        Vertex {
            position: [-0.5, -0.5, 0.0],
            speed: 0.0,
        },
        Vertex {
            position: [0.0, 0.5, 0.0],
            speed: 0.0,
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            speed: 0.0,
        },
    ];

    let bytes: &[u8] = bytemuck::cast_slice(&vertices);
    assert_eq!(bytes.len(), 48);
}
#[test]
fn vertex_layout() {
    let layout = Vertex::layout();
    assert_eq!(layout.array_stride, 16);
    assert_eq!(layout.attributes.len(), 2);

    assert_eq!(layout.attributes[0].shader_location, 0);
    assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(layout.attributes[0].offset, 0);

    assert_eq!(layout.attributes[1].shader_location, 1);
    assert_eq!(layout.attributes[1].format, wgpu::VertexFormat::Float32);
    assert_eq!(layout.attributes[1].offset, 12);
}
