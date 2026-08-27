mod rays;

use glam::{Mat3, Vec3};

use crate::common::{Angle, Camera, Node, Splat};

mod common;

fn main() {
    let mut cam = Camera::new(
        400,
        400,
        0.7,
        Node::new(Vec3::new(0., 0., 2.), Angle::new(0.0, 0.0)),
    );

    let mut splats = vec![
        Splat {
            position: Vec3::new(10., 0., 0.),
            scale: Mat3::IDENTITY,
            color: |_c| Vec3::new(1., 0., 0.),
        },
        Splat {
            position: Vec3::new(15., 0., 0.),
            scale: Mat3::IDENTITY,
            color: |_c| Vec3::new(0., 1., 0.),
        },
    ];

    // let splats2 = vec![
    //     Splat {
    //         position: Vec3::new(10., 0., 0.),
    //         scale: Mat3::IDENTITY,
    //         color: |_c| Vec3::new(0., 1., 0.),
    //     },
    //     Splat {
    //         position: Vec3::new(15., 0., 0.),
    //         scale: Mat3::IDENTITY,
    //         color: |_c| Vec3::new(1., 0., 0.),
    //     },
    // ];

    for index in 0..8 {
        println!("Starting with {index}");
        // cam.node.position += Vec3::Y * (10. / 24.);
        cam.render(&splats);
        cam.write(&index).unwrap();
        println!("Completed with {index}");
        splats[0].position += 0.5 * Vec3::Z;
    }

    // println!("{cam:?}");
    // println!("{:?}", cam.node.angle.to_vec());

    // let r = Ray::from_camera(&cam, [1., 0.]);
    // println!("{:?}", r);

    // let output_color = r.render_gaussian(splats);
    // println!("{output_color:?}");
}
