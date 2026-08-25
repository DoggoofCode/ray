mod common;
use common::{Angle, Camera, Cartesian, RGB, Splat};

fn main() {
    let x = |_: Angle| RGB::new(255, 0, 0);
    let splat = Splat {
        position: Cartesian::new(0., 0., 0.),
        scale: Cartesian::new(1., 1., 1.),
        color: x,
    };
    println!("{:?}", splat);

    let mut screen = Camera::new(
        8,
        6,
        1.0,
        Cartesian {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    );
    println!("{:?}", screen);
}
