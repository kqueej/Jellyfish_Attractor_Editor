use jellyfish::graphics::vertex::Vertex;

#[test]
fn vertex_size() {
    assert_eq!(std::mem::size_of::<Vertex>(), 12);
}

#[test]
fn vertex_bytes() {
    let vertices = [
        Vertex {
            position: [-0.5, -0.5, 0.0],
        },
        Vertex {
            position: [0.0, 0.5, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
        },
    ];

    let bytes: &[u8] = &bytemuck::cast_slice(&vertices);
    assert_eq!(bytes.len(), 36);
}
#[test]
fn vertex_layout() {
    let layout = Vertex::layout();
    assert_eq!(layout.array_stride, 12);
    assert_eq!(layout.attributes.len(), 1);
    assert_eq!(layout.attributes[0].offset, 0);
    assert_eq!(layout.attributes[0].shader_location, 0);
    assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x3);
}
