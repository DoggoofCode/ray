use std::{
    f32::consts::PI,
    io::{self, Write},
    time::Instant,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};
use glam::Vec3;

use crate::common::{Camera, Splat};

fn push_u8(buf: &mut Vec<u8>, n: u8) {
    if n >= 100 {
        buf.push(b'0' + n / 100);
        buf.push(b'0' + (n / 10) % 10);
        buf.push(b'0' + n % 10);
    } else if n >= 10 {
        buf.push(b'0' + n / 10);
        buf.push(b'0' + n % 10);
    } else {
        buf.push(b'0' + n);
    }
}

fn push_foreground(buf: &mut Vec<u8>, rgb: (u8, u8, u8)) {
    buf.extend_from_slice(b"\x1b[38;2;");
    push_u8(buf, rgb.0);
    buf.push(b';');
    push_u8(buf, rgb.1);
    buf.push(b';');
    push_u8(buf, rgb.2);
    buf.push(b'm');
    buf.extend_from_slice("▀".as_bytes());
}

fn push_background(buf: &mut Vec<u8>, rgb: (u8, u8, u8)) {
    buf.extend_from_slice(b"\x1b[48;2;");
    push_u8(buf, rgb.0);
    buf.push(b';');
    push_u8(buf, rgb.1);
    buf.push(b';');
    push_u8(buf, rgb.2);
    buf.push(b'm');
}

fn get_screen_size(virtual_screen: &Camera) -> std::io::Result<((f32, f32), u16, f32)> {
    let (width, mut height) = crossterm::terminal::size()?;
    height *= 2;
    let virtual_aspect_ratio = virtual_screen.width as f32 / virtual_screen.height as f32;
    let aspect_ratio = width as f32 / height as f32;

    let scale_ratio: f32;
    let (real_image_width, real_image_height): (f32, f32);
    if aspect_ratio < virtual_aspect_ratio {
        // Use width
        scale_ratio = virtual_screen.width as f32 / width as f32; // Number of virtual pixels per real pixel
        println!("{scale_ratio}");
        (real_image_width, real_image_height) =
            (width as f32, virtual_screen.height as f32 / scale_ratio);
    } else {
        // Use height
        scale_ratio = virtual_screen.height as f32 / height as f32; // Number of virtual pixels per real pixel
        (real_image_width, real_image_height) =
            (virtual_screen.width as f32 / scale_ratio, height as f32);
        println!("{scale_ratio}");
    }
    let kernel_size: u16 = aspect_ratio as u16;
    Ok((
        (real_image_width, real_image_height),
        kernel_size,
        scale_ratio,
    ))
}

fn render_from_camera(
    virtual_screen: &Camera,
    image_buffer: &mut Vec<u8>,
    (real_image_width, real_image_height): (f32, f32),
    kernel_size: u16,
    scale_ratio: f32,
) {
    for image_height in (0..real_image_height as u16).step_by(2) {
        for image_width in 0..real_image_width as u16 {
            let (mut upper_r, mut upper_g, mut upper_b): (u16, u16, u16) = (0, 0, 0);
            let (mut lower_r, mut lower_g, mut lower_b): (u16, u16, u16) = (0, 0, 0);
            for kernel_x in 0..kernel_size {
                for kernel_y in 0..kernel_size {
                    let upper_color = virtual_screen.screen[virtual_screen.width as usize
                        * ((image_height * scale_ratio as u16) + kernel_y) as usize
                        + (image_width * scale_ratio as u16) as usize
                        + kernel_x as usize];
                    upper_r += upper_color.r as u16;
                    upper_g += upper_color.g as u16;
                    upper_b += upper_color.b as u16;
                    let lower_color = virtual_screen.screen[virtual_screen.width as usize
                        * (((image_height + 1) * scale_ratio as u16) + kernel_y) as usize
                        + (image_width * scale_ratio as u16) as usize
                        + kernel_x as usize];
                    lower_r += lower_color.r as u16;
                    lower_g += lower_color.g as u16;
                    lower_b += lower_color.b as u16;
                }
            }
            let k_squared = kernel_size * kernel_size;
            upper_r /= k_squared;
            upper_g /= k_squared;
            upper_b /= k_squared;
            lower_r /= k_squared;
            lower_g /= k_squared;
            lower_b /= k_squared;
            push_background(image_buffer, (lower_r as u8, lower_g as u8, lower_b as u8));
            push_foreground(image_buffer, (upper_r as u8, upper_g as u8, upper_b as u8));
        }
        image_buffer.extend_from_slice(b"\x1b[0m");
        image_buffer.extend_from_slice(b"\n\r");
    }
}

pub fn render(virtual_screen: &mut Camera, splats: &[Splat]) -> std::io::Result<()> {
    let ((real_image_width, real_image_height), kernel_size, scale_ratio) =
        get_screen_size(virtual_screen)?;

    let mut image_buffer: Vec<u8> =
        Vec::with_capacity(real_image_height as usize * real_image_width as usize * 16);

    let mut stdout = io::stdout();

    // Setup
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // One buffer for the entire frame
    let mut buffer: Vec<u8> = Vec::with_capacity(64 * 1024);

    let mut last_frame = Instant::now();

    loop {
        if event::poll(std::time::Duration::from_millis(10))?
            && let Event::Key(key) = event::read()?
        {
            if key.code == KeyCode::Esc {
                break;
            }
            if key.code == KeyCode::Char('w') {
                virtual_screen.node.position += Vec3::X;
            }
            if key.code == KeyCode::Char('s') {
                virtual_screen.node.position -= Vec3::X;
            }
            if key.code == KeyCode::Char('a') {
                virtual_screen.node.position += Vec3::Y;
            }
            if key.code == KeyCode::Char('d') {
                virtual_screen.node.position -= Vec3::Y;
            }
            if key.code == KeyCode::Char(' ') {
                virtual_screen.node.position += Vec3::Z;
            }
            if key.code == KeyCode::Tab {
                virtual_screen.node.position -= Vec3::Z;
            }
            if key.code == KeyCode::Up {
                virtual_screen.node.angle.phi += PI / 16.;
            }
            if key.code == KeyCode::Down {
                virtual_screen.node.angle.phi -= PI / 16.;
            }
            if key.code == KeyCode::Left {
                virtual_screen.node.angle.theta += PI / 16.;
            }
            if key.code == KeyCode::Right {
                virtual_screen.node.angle.theta -= PI / 16.;
            }
        }

        buffer.clear();

        virtual_screen.render(splats);

        // Set FPS
        let now = Instant::now();
        let frame_time = now - last_frame;
        last_frame = now;

        let fps = 1.0 / frame_time.as_secs_f64();

        let fps = format!(
            "\x1b[0mFPS: {:.1} | frame: {:.2} ms | {:?}",
            fps,
            frame_time.as_secs_f64() * 1000.0,
            virtual_screen.node
        );

        render_from_camera(
            virtual_screen,
            &mut image_buffer,
            (real_image_width, real_image_height),
            kernel_size,
            scale_ratio,
        );

        buffer.extend_from_slice(&image_buffer);
        buffer.extend_from_slice(fps.as_bytes());

        image_buffer.clear();

        // One syscall/write
        execute!(stdout, MoveTo(0, 0), Clear(terminal::ClearType::All))?;
        stdout.write_all(&buffer)?;
        stdout.flush()?;

        // input / timing...
    }

    // Cleanup
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

/* #[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render() {
        render().unwrap();
    }
} */
