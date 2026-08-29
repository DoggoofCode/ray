mod rays;

use std::env;
use std::f32::consts::{PI, TAU};

use glam::{Mat3, Vec3, mat3};
use libm::{cosf, fabsf, sinf};

use crate::ascii::render;
use crate::common::{Angle, Camera, Node, Splat};

mod common;

mod fastfunclib;

mod ascii;

fn main() {
    let mut cam = Camera::new(
        800,
        800,
        0.5,
        Node::new(Vec3::new(0., 0., 0.), Angle::new(0.0, 0.0)),
    );

    let splats = vec![
        Splat::new(
            Vec3::new(15., 0., 0.),
            mat3(
                Vec3::new(4., 0., 0.),
                Vec3::new(0., 4., 0.),
                Vec3::new(0., 0., 4.),
            ),
            |c: Angle| -> Vec3 {
                let mod_theta = fabsf(c.theta) % TAU;
                Vec3::new(0., 1. - mod_theta / TAU, mod_theta / TAU)
            },
        ),
        Splat {
            position: Vec3::new(10., 0., 0.),
            sigma_inv: Mat3::IDENTITY,
            color: |c: Angle| -> Vec3 {
                let mod_theta = fabsf(c.theta) % TAU;
                Vec3::new(1. - mod_theta / TAU, mod_theta / TAU, 0.)
            },
        },
    ];

    if env::args().len() > 1 {
        cam.render(&splats);
        render(&cam).unwrap();
    } else {
        // Non-realtime ppm rendering
        let mut percentage: f32;
        let frames = 48;
        for index in 0..frames {
            percentage = index as f32 / frames as f32;
            cam.node.angle = Angle::new(percentage * TAU, -0.56);
            cam.node.position = Vec3::new(
                12.5 + 12.5 * cosf(TAU * percentage + PI),
                12.5 * sinf(TAU * percentage + PI),
                8.,
            );
            cam.render(&splats);
            // cam.write(&index).unwrap();
            println!("Completed with {index}");
        }
    }
}
