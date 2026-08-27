use std::f32::consts::PI;

use crate::{
    common::{Angle, Camera, Color, Splat},
    fastfunclib::{Erfc, Expf},
};
use glam::Vec3;
use libm::sqrtf;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub struct Abc {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Ray {
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

    fn gaussian_coefficients(&self, splat: &Splat) -> Abc {
        let r = self.direction;
        let d = self.origin - splat.position;

        let sigma_inv_d = splat.sigma_inv * d;
        let sigma_inv_r = splat.sigma_inv * r;

        let a = r.dot(sigma_inv_r);
        let b = r.dot(sigma_inv_d);
        let c = d.dot(sigma_inv_d);
        Abc { a, b, c }
    }

    // Order: [3, 0, 1, 2] => 3 is closest, 0 is second, etc. etc.
    pub fn render_gaussian(
        &self,
        splats: &[Splat],
        erfc: &Erfc,
        expf: &Expf,
        abc_values: &mut Vec<Abc>,
        order: &mut Vec<u16>,
        distance: &mut Vec<f32>,
    ) -> Color {
        let mut t = 1.;
        let mut pixel = Vec3::ZERO;
        let mut integrated_density;

        for splat in splats.iter() {
            abc_values.push(self.gaussian_coefficients(splat));
        }

        // Calculate depth

        for (splat_index, Abc { a, b, .. }) in abc_values.iter().enumerate() {
            let t_star = -b / a;
            let mut head = 0;
            if order.is_empty() {
                order.push(splat_index as u16);
                distance.push(t_star);
                continue;
            }
            while head < order.len() && distance[head] < t_star {
                head += 1;
            }
            order.insert(head, splat_index as u16);
            distance.insert(head, t_star);
        }
        // println!("{order:?} {distance:?}");

        for splat_index in order.iter() {
            let gsplat = splats[(*splat_index) as usize];
            // println!("{:?}, {:?}", gsplat, order);
            let Abc { a, b, c } = abc_values[(*splat_index) as usize];
            integrated_density = sqrtf(PI / (2. * a))
                * expf.eval(-0.5 * (c - (b * b / a)))
                * erfc.eval(b / sqrtf(2. * a));
            let alpha = 1. - expf.eval(-integrated_density);

            pixel += t * alpha * (gsplat.color)(Angle::from_vec(self.direction));

            t *= 1. - alpha;
            // println!("{:?}, {t}", pixel)
        }

        abc_values.clear();
        order.clear();
        distance.clear();
        Color::from_vec(pixel)
    }
}
