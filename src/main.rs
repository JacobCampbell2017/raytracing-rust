mod color;
mod vec3;
use color::*;
use vec3::Vec3;

// Left off starting chapter 4

fn main() {
    // Image
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    // render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..(image_height) {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..(image_width) {
            let pixel_color = Color::new_use(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            write_color(pixel_color);
        }
    }
    eprintln!("\rDone.              ");

    /*
    let mut vector1 = Color::new();
    vector1[0] = 3.0;
    vector1[1] = 2.0;
    vector1[2] = 1.0;
    let mut vector2 = Color::new_use(1.0, 2.0, 3.0);
    let neg_vec = -vector2.clone();
    eprintln!("{}\t{} {}", vector2, neg_vec, vector1);
    */
}
