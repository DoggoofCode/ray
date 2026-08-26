use std::fs::File;
use std::io::Write;

use glam::{Mat3, Vec3};

use crate::rays::Ray;

#[derive(Debug)]
// All cameras have a screen of 1m
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub focal_distance: f32,
    pub screen: Vec<Vec<RGB>>,
    pub node: Node,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub position: Vec3,
    pub angle: Angle,
}

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub alpha: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, alpha: f32) -> Self {
        Color { r, g, b, alpha }
    }

    pub fn from_vec(v: Vec3) -> Self {
        Color {
            r: v.x,
            g: v.y,
            b: v.z,
            alpha: 1.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Splat {
    pub position: Vec3,
    pub scale: Mat3,
    pub color: fn(Angle) -> Vec3,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB {
            r,
            g,
            b,
            alpha: 255,
        }
    }

    pub fn from_color(color: Color) -> Self {
        RGB {
            r: (color.r * 255.) as u8,
            g: (color.g * 255.) as u8,
            b: (color.b * 255.) as u8,
            alpha: 255,
        }
    }
}

impl Node {
    pub fn new(position: Vec3, angle: Angle) -> Self {
        Self { position, angle }
    }
}

impl Camera {
    pub fn new(width: u32, height: u32, focal_distance: f32, node: Node) -> Self {
        let screen = vec![vec![RGB::new(0, 0, 0); width as usize]; height as usize];
        Camera {
            width,
            height,
            focal_distance,
            screen,
            node,
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }

    pub fn render(&mut self, splats: Vec<Splat>) {
        let mut width;
        let mut height;
        let mut r: Ray;
        for h in 0..self.height {
            for w in 0..self.width {
                width = (2. * (w as f32) / (self.width as f32)) - 1.;
                height = (2. * (h as f32) / (self.height as f32)) - 1.;
                r = Ray::from_camera(self, [width, height]);
                self.screen[h as usize][w as usize] = RGB::from_color(r.render_gaussian(&splats));
            }
        }
    }

    pub fn write(&self) -> std::io::Result<()> {
        let mut file = File::create("frames/frame0001.ppm")?;
        writeln!(file, "P3")?;
        writeln!(file, "{} {}\n255", self.width, self.height)?;
        for h in 0..self.height {
            for w in 0..self.width {
                let pixel_value = self.screen[h as usize][w as usize];
                writeln!(
                    file,
                    "{} {} {}",
                    pixel_value.r, pixel_value.g, pixel_value.b
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Angle {
    pub theta: f32,
    pub phi: f32,
}

impl Angle {
    pub fn new(theta: f32, phi: f32) -> Self {
        Angle { theta, phi }
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(
            self.phi.cos() * self.theta.cos(),
            self.phi.cos() * self.theta.sin(),
            self.phi.sin(),
        )
    }

    pub fn from_vec(v: Vec3) -> Self {
        let v = v.normalize();

        Self {
            theta: v.y.atan2(v.x),
            phi: v.z.asin(),
        }
    }
}
