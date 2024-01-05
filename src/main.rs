use linuxfb;

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

    // Define your color in RGB:
    let red = 0xBF; // Adjust the intensity of red (0 to 255)
    let green = 0xAF; // Adjust the intensity of green (0 to 255)
    let blue = 0xCF; // Adjust the intensity of blue (0 to 255)

    // Assuming a 32-bit RGBA format (8 bits for each of red, green, blue, alpha)
    let color = (red as u32) << 16 | (green as u32) << 8 | (blue as u32);

    // Modify the loop to set this color:
    loop {
        for i in (0..data.len()).step_by(4) {
            let pixel = &mut data[i..i + 4];

            pixel[0] = (color & 0xFF) as u8; // Blue
            pixel[1] = ((color >> 8) & 0xFF) as u8; // Green
            pixel[2] = ((color >> 16) & 0xFF) as u8; // Red
            //pixel[3] = 0xAF; // Alpha (0xFF for fully opaque)
        }

        for i in 0..data.len() {
            data[i] = 0xFF;
        }
    }
}
