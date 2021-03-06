//use three;
use rand;

pub fn random_tuple(x_extent: f32, y_extent: f32) -> (f32, f32) {
    let x = rand::random::<f32>() * 2.0 * x_extent - x_extent;
    let y = rand::random::<f32>() * 2.0 * y_extent - y_extent;

    (x,y)
}

pub fn random_in_range(x: f32, y: f32) -> f32 {
    rand::random::<f32>() * (y-x) + x
}

//pub fn create_quad(window: &mut three::Window, scale: [f32; 2]) -> three::Mesh {
//    let vertices = vec![
//        [-1.0, -1.0, 0.0].into(),
//        [1.0, -1.0, 0.0].into(),
//        [1.0, 1.0, 0.0].into(),
//        [-1.0, 1.0, 0.0].into(),
//    ];
//
//    let faces = vec![[0, 1, 2], [2, 3, 0]];
//
//    let mut quad = three::Geometry {
//        faces,
//        base: three::Shape {
//            vertices,
//            ..three::Shape::default()
//        },
//        ..three::Geometry::default()
//    };
//
//    for v in quad.base.vertices.iter_mut() {
//        v.x *= scale[0];
//        v.y *= scale[1];
//    }
//
//    let material = three::material::Line { color: 0x000000 };
//
//    let mesh = window.factory.mesh(quad, material);
//
//    mesh
//}

pub fn create_circle(window: &mut three::Window, segments: u32) -> three::Mesh {
    let segment_angle = 2.0 * std::f32::consts::PI / segments as f32;

    let mut vertices = Vec::new();

    let mut cur_angle: f32 = 0.0;

    vertices.push([0.0, 0.0, 0.0].into());
    for _ in 0..segments {
        vertices.push([cur_angle.sin(), cur_angle.cos(), 0.0].into());

        cur_angle += segment_angle;
    }

    let mut faces = Vec::new();

    for i in 0..segments + 1 {
        faces.push([0, (i + 2) % segments + 1, (i + 1) % segments + 1]);
    }

    let circle = three::Geometry {
        faces,
        base: three::Shape {
            vertices,
            ..three::Shape::default()
        },
        ..three::Geometry::default()
    };

    let material = three::material::Wireframe {
        color: 0x000000,
    };

    let mesh = window.factory.mesh(circle, material);

    mesh
}
