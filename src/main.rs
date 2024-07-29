use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;
use image::{ImageBuffer, Rgb};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn save_framebuffer_as_bmp(buffer: &Vec<u32>, width: usize, height: usize, filename: &str) {
    let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let idx = (y as usize * width + x as usize) as usize;
        let color = buffer[idx];
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        *pixel = Rgb([r, g, b]);
    }

    imgbuf.save(filename).unwrap();
}

fn count_neighbors(buffer: &Vec<u32>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx < WIDTH && ny < HEIGHT {
                if buffer[ny * WIDTH + nx] == 0xFFFFFF {
                    count += 1;
                }
            }
        }
    }
    count
}

fn next_generation(buffer: &mut Vec<u32>) {
    let mut new_buffer = buffer.clone();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbors = count_neighbors(buffer, x, y);
            let idx = y * WIDTH + x;
            if buffer[idx] == 0xFFFFFF {
                if neighbors < 2 || neighbors > 3 {
                    new_buffer[idx] = 0x000000; // Cell dies
                }
            } else {
                if neighbors == 3 {
                    new_buffer[idx] = 0xFFFFFF; // Cell becomes alive
                }
            }
        }
    }
    *buffer = new_buffer;
}

fn render(framebuffer: &mut Framebuffer, buffer: &Vec<u32>) {
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();
    framebuffer.set_current_color(0xFFFFFF);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if buffer[y * WIDTH + x] == 0xFFFFFF {
                framebuffer.point(x, y);
            }
        }
    }
}

fn initialize_pulsar(buffer: &mut Vec<u32>, x: usize, y: usize) {
    let pulsar_coords = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];

    for (dx, dy) in pulsar_coords.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx < WIDTH && ny < HEIGHT {
            buffer[ny * WIDTH + nx] = 0xFFFFFF;
        }
    }
}

fn initialize_lwss(buffer: &mut Vec<u32>, x: usize, y: usize) {
    let lwss_coords = [
        (1, 0), (4, 0),
        (0, 1), (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3),
    ];

    for (dx, dy) in lwss_coords.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx < WIDTH && ny < HEIGHT {
            buffer[ny * WIDTH + nx] = 0xFFFFFF;
        }
    }
}

fn initialize_block(buffer: &mut Vec<u32>, x: usize, y: usize) {
    let block_coords = [
        (0, 0), (1, 0),
        (0, 1), (1, 1),
    ];

    for (dx, dy) in block_coords.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx < WIDTH && ny < HEIGHT {
            buffer[ny * WIDTH + nx] = 0xFFFFFF;
        }
    }
}

fn initialize_beehive(buffer: &mut Vec<u32>, x: usize, y: usize) {
    let beehive_coords = [
        (1, 0), (2, 0),
        (0, 1), (3, 1),
        (1, 2), (2, 2),
    ];

    for (dx, dy) in beehive_coords.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx < WIDTH && ny < HEIGHT {
            buffer[ny * WIDTH + nx] = 0xFFFFFF;
        }
    }
}

fn initialize_pentadecathlon(buffer: &mut Vec<u32>, x: usize, y: usize) {
    let pentadecathlon_coords = [
        (2, 0), (3, 0), (4, 0),
        (0, 2), (5, 2),
        (0, 3), (5, 3),
        (0, 4), (5, 4),
        (2, 6), (3, 6), (4, 6),
    ];

    for (dx, dy) in pentadecathlon_coords.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx < WIDTH && ny < HEIGHT {
            buffer[ny * WIDTH + nx] = 0xFFFFFF;
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 800;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Rust Graphics - Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut buffer = vec![0x000000; WIDTH * HEIGHT];

    // Initialize patterns at specific coordinates
    initialize_pulsar(&mut buffer, 10, 10);
    initialize_pulsar(&mut buffer, 40, 10);
    initialize_pulsar(&mut buffer, 70, 10);


    initialize_pulsar(&mut buffer, 10, 40);
    initialize_pulsar(&mut buffer, 40, 40);
    initialize_pulsar(&mut buffer, 70, 40);


    initialize_pulsar(&mut buffer, 10, 70);
    initialize_pulsar(&mut buffer, 40, 70);
    initialize_pulsar(&mut buffer, 70, 70);

    initialize_lwss(&mut buffer, 70, 25);
    initialize_lwss(&mut buffer, 70, 55);


    initialize_pentadecathlon(&mut buffer, 10, 25);





    while window.is_open() {
        // Listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Take a screenshot if 'S' key is pressed
        if window.is_key_down(Key::S) {
            save_framebuffer_as_bmp(&buffer, WIDTH, HEIGHT, "screenshot.bmp");
        }

        // Render current generation
        render(&mut framebuffer, &buffer);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        // Calculate next generation
        next_generation(&mut buffer);

        std::thread::sleep(frame_delay);
    }
}
