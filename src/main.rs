mod rays;

use glam::{Mat3, Vec3};

use crate::common::{Angle, Camera, Node, Splat};

mod common;

fn main() {
    let mut cam = Camera::new(
        800,
        600,
        1.,
        Node::new(Vec3::new(0., 0., 0.), Angle::new(0.0, 0.0)),
    );

    let splats = vec![Splat {
        position: Vec3::new(15., 0., 0.),
        scale: Mat3::IDENTITY,
        color: |_c| Vec3::new(1., 0., 0.),
    }];

    cam.render(splats);
    cam.write().unwrap();

    // println!("{cam:?}");
    // println!("{:?}", cam.node.angle.to_vec());

    // let r = Ray::from_camera(&cam, [1., 0.]);
    // println!("{:?}", r);

    // let output_color = r.render_gaussian(splats);
    // println!("{output_color:?}");
}
