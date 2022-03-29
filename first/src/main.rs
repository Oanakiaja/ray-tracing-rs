use ray::Ray;
use vec3::{Color, Point3, Vec3};

mod ray;
mod vec3;

use image::{ImageBuffer, Rgb};

fn ray_color(ray: &Ray) -> [u8; 3] {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let color = (1.0 - t) * Color::from(0.5, 0.7, 1.0) + (t) * Color::from(1.0, 1.0, 1.0);
    // blendedValue= (1−𝑡)⋅startValue + 𝑡⋅endValue
    [
        (color.x() * 255.0) as u8,
        (color.y() * 255.0) as u8,
        (color.z() * 255.0) as u8,
    ]
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width / aspect_ratio as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - Vec3::from(0.0, 0.0, focal_length);

    let mut image = ImageBuffer::new(image_width as u32, image_height as u32);

    // Render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let ray = Ray {
                origin: origin.clone(),
                direction: (lower_left_corner.clone()
                    + u * horizontal.clone()
                    + v * vertical.clone()
                    - origin.clone()),
            };
            let color = ray_color(&ray);
            image.put_pixel(i as u32, j as u32, Rgb(color));
        }
    }
    image.save("result.png").unwrap();
}
