mod color;
mod ray;
mod vec3;
use color::*;
use ray::*;
use vec3::Vec3;

// Left off 6.2

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate image height - Ensure its at least 1
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / image_height as f64);
    let camera_center = Point3::new_use(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new_use(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new_use(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new_use(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..(image_height) {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..(image_width) {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new_use(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r);
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
