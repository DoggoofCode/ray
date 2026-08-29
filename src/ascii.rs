use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::common::Camera;

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

pub fn render(virtual_screen: &Camera) -> std::io::Result<()> {
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

    let mut image_buffer: Vec<u8> =
        Vec::with_capacity(real_image_height as usize * real_image_width as usize * 16);
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
            push_background(
                &mut image_buffer,
                (lower_r as u8, lower_g as u8, lower_b as u8),
            );
            push_foreground(
                &mut image_buffer,
                (upper_r as u8, upper_g as u8, upper_b as u8),
            );
        }
        image_buffer.extend_from_slice(b"\x1b[0m");
        image_buffer.extend_from_slice(b"\n\r");
    }

    // panic!("asdf");

    let mut stdout = io::stdout();

    // Setup
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // One buffer for the entire frame
    let mut buffer: Vec<u8> = Vec::with_capacity(64 * 1024);

    loop {
        if event::poll(std::time::Duration::from_millis(10))?
            && let Event::Key(key) = event::read()?
            && key.code == KeyCode::Esc
        {
            break;
        }

        buffer.clear();

        // Build frame
        execute!(stdout, MoveTo(0, 0), Clear(terminal::ClearType::All))?;
        // buffer.extend_from_slice(b"\x1b[2J\x1b[H");
        buffer.extend_from_slice(&image_buffer);

        // One syscall/write
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
