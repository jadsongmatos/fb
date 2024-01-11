use linuxfb::Framebuffer;
use std::{ptr, thread, time};

fn main() {
    let devices = Framebuffer::list().unwrap();
    println!("Devices: {:?}", devices);

    let fb = Framebuffer::new("/dev/fb0").unwrap();
    println!("Size in pixels: {:?}", fb.get_size());
    println!("Bytes per pixel: {:?}", fb.get_bytes_per_pixel());
    println!("Physical size in mm: {:?}", fb.get_physical_size());

    let frame_duration = time::Duration::from_millis(1000 / 3); // Duração de um quadro para 3 FPS

    let mut display_buffer = fb.map().unwrap();
    let mut offscreen_buffer = vec![0; display_buffer.len()];

    let red: u8 = 0xBF;
    let green: u8 = 0xAF;
    let blue: u8 = 0xCF;

    let mut cor = false;

    loop {
        let start = time::Instant::now(); // Marca o início do quadro

        // Draw to the off-screen buffer
        for pixel in offscreen_buffer.chunks_exact_mut(4) {
            if cor {
                pixel[0] = blue;
                pixel[1] = green;
                pixel[2] = red;
            } else {
                pixel[0] = 0xFF;
                pixel[1] = 0xFF;
                pixel[2] = 0xFF;
            }
        }

        cor = !cor;

        // Swap buffers (copy off-screen buffer to the display buffer)
        //display_buffer.copy_from_slice(&offscreen_buffer);
        unsafe {
            ptr::copy_nonoverlapping(offscreen_buffer.as_ptr(), display_buffer.as_mut_ptr(), offscreen_buffer.len());
        }

        let elapsed = start.elapsed(); // Calcula o tempo gasto na renderização
        if elapsed < frame_duration {
            // Se a renderização foi mais rápida do que a duração de um quadro, pausa o loop
            thread::sleep(frame_duration - elapsed);
        } else {
            println!("lento: {:?}",elapsed);
        }

        println!("frame_duration: {:?}",elapsed);
    }
}
