use std::f32::consts::PI;

use crate::common::{Angle, Camera, Color, Splat};
use glam::Vec3;
use libm::{erfcf, expf, sqrtf};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    // Screen loc: [-1, 1] & [-1, 1] for x and y
    pub fn from_camera(camera: &Camera, screen_loc: [f32; 2]) -> Self {
        // Known vectors
        let forward = camera.node.angle.to_vec();
        let world_up = Vec3::Z;

        // Screen basis vectors
        let right = forward.cross(world_up).normalize();
        let up = right.cross(forward).normalize();

        // Screen sizes
        let screen_width = 1.0;
        let screen_height: f32 = screen_width / camera.aspect_ratio();

        let screen_center: Vec3 = camera.node.position + forward * camera.focal_distance;

        // Relative to the screen
        let x = screen_loc[0] * screen_width / 2.0;
        let y = screen_loc[1] * screen_height / 2.0;

        let screen_point = screen_center + right * x + up * y;

        let direction = (screen_point - camera.node.position).normalize();

        Self {
            origin: camera.node.position,
            direction,
        }
    }

    fn gaussian_coefficients(&self, splat: &Splat) -> (f32, f32, f32) {
        let r = self.direction;
        let d = self.origin - splat.position;

        let sigma_inv_d = splat.scale * d;
        let sigma_inv_r = splat.scale * r;

        let a = r.dot(sigma_inv_r);
        let b = r.dot(sigma_inv_d);
        let c = d.dot(sigma_inv_d);
        (a, b, c)
    }

    pub fn render_gaussian(&self, splats: &Vec<Splat>) -> Color {
        let mut T = 1.;
        let mut pixel = Vec3::ZERO;

        let gsplat = splats[0];
        let (a, b, c) = self.gaussian_coefficients(&gsplat);
        let mut integrated_density =
            sqrtf(PI / (2. * a)) * expf(-0.5 * (c - (b * b / a))) * erfcf(b / sqrtf(2. * a));
        let alpha = 1. - expf(-integrated_density);

        pixel += T * alpha * (gsplat.color)(Angle::from_vec(self.direction));

        T *= 1. - alpha;

        Color::from_vec(pixel)
    }
}

pub trait Erfc {
    fn erfc(self) -> Self;
}

impl Erfc for Vec3 {
    #[inline]
    fn erfc(self) -> Self {
        Vec3::new(
            libm::erfcf(self.x),
            libm::erfcf(self.y),
            libm::erfcf(self.z),
        )
    }
}
