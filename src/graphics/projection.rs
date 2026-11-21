use super::vertex::Vertex;

pub fn project_xy(points: &[[f64; 3]]) -> Vec<Vertex> {
    if points.is_empty() {
        return Vec::new();
    }

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for point in points {
        min_x = min_x.min(point[0]);
        max_x = max_x.max(point[0]);
        min_y = min_y.min(point[1]);
        max_y = max_y.max(point[1]);
    }

    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    let width = max_x - min_x;
    let height = max_y - min_y;
    let span = width.max(height);

    let scale = if span > 0.0 { 1.8 / span } else { 1.0 };

    points
        .iter()
        .map(|point| Vertex {
            position: [
                ((point[0] - center_x) * scale) as f32,
                ((point[1] - center_y) * scale) as f32,
                0.0,
            ],
        })
        .collect()
}

pub fn normalize_xyz(points: &[[f64; 3]]) -> Vec<Vertex> {
    if points.is_empty() {
        return Vec::new();
    }

    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];

    for point in points {
        for axis in 0..3 {
            min[axis] = min[axis].min(point[axis]);
            max[axis] = max[axis].max(point[axis]);
        }
    }

    let mut center = [0.0; 3];
    let mut span: f64 = 0.0;

    for axis in 0..3 {
        center[axis] = (min[axis] + max[axis]) / 2.0;
        span = span.max(max[axis] - min[axis]);
    }

    let scale = if span > 0.0 { 1.8 / span } else { 1.0 };
    points
        .iter()
        .map(|point| Vertex {
            position: [
                ((point[0] - center[0]) * scale) as f32,
                ((point[1] - center[1]) * scale) as f32,
                ((point[2] - center[2]) * scale) as f32,
            ],
        })
        .collect()
}
