```rust

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

    // Modify the loop to set this color:
    loop {     
        for i in 0..data.len() {
            data[i] = 0xFF;
        }

    }
}
```
