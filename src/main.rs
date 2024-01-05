use linuxfb;
use std::{thread, time};

fn main(){
    let devices = linuxfb::Framebuffer::list().unwrap();
    println!("Devices: {:?}", devices);

    // Instead of hardcoding the path, you could also use `Framebuffer::list()`
    // to find paths to available devices.
    let fb = linuxfb::Framebuffer::new("/dev/fb0").unwrap();

    println!("Size in pixels: {:?}", fb.get_size());

    println!("Bytes per pixel: {:?}", fb.get_bytes_per_pixel());

    println!("Physical size in mm: {:?}", fb.get_physical_size());

    // Map the framebuffer into memory, so we can write to it:
    let mut data = fb.map().unwrap();

    //1049088 1366x768
    //4196352 1366x768x4
    println!("data: {:?}",data.len());


    let frame_duration = time::Duration::from_millis(33); // Duração de um quadro para 30 FPS

    // Define your color in RGB:
    let red: u8 = 0xBF; // Adjust the intensity of red (0 to 255)
    let green: u8 = 0xAF; // Adjust the intensity of green (0 to 255)
    let blue: u8 = 0xCF; // Adjust the intensity of blue (0 to 255)

    // Modify the loop to set this color:
    loop {
        let start = time::Instant::now(); // Marca o início do quadro

        for pixel in data.chunks_exact_mut(4) {
            pixel[0] = blue;
            pixel[1] = green;
            pixel[2] = red;
        }
      
        let elapsed = start.elapsed(); // Calcula o tempo gasto na renderização

        if elapsed < frame_duration {
            // Se a renderização foi mais rápida do que a duração de um quadro, pausa o loop
            thread::sleep(frame_duration - elapsed);
            
        } else {
            println!("frame_duration: {:?}",elapsed)
        }
    }
}
