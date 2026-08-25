#[derive(Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub focal_distance: f32,
    pub screen: Vec<Vec<RGB>>,
    pub position: Cartesian,
}

#[derive(Debug, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone)]
pub struct Cartesian {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct Angle {
    pub azimuth: f32,
    pub theta: f32,
}

#[derive(Debug)]
pub struct Splat {
    pub position: Cartesian,
    pub scale: Cartesian,
    pub color: fn(Angle) -> RGB,
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
}

impl Cartesian {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Cartesian { x, y, z }
    }
}

impl Camera {
    pub fn new(width: u32, height: u32, focal_distance: f32, position: Cartesian) -> Self {
        let screen = vec![vec![RGB::new(0, 0, 0); width as usize]; height as usize];
        Camera {
            width,
            height,
            focal_distance,
            screen,
            position,
        }
    }
}

impl Angle {
    pub fn new(azimuth: f32, theta: f32) -> Self {
        Angle { azimuth, theta }
    }
}
