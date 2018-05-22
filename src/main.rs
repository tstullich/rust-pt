extern crate image;

mod ray;
mod vector;

use std::path::Path;

fn color(r: ray::Ray) -> vector::Vec3 {
    let unit_direction = r.direction().unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    vector::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) +
        vector::Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let lower_left_corner = vector::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vector::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vector::Vec3::new(0.0, 2.0, 0.0);
    let origin = vector::Vec3::new(0.0, 0.0, 0.0);

    let img_x = 200;
    let img_y = 100;
    let mut imgbuf = image::ImageBuffer::new(img_x, img_y);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / img_x as f32;
        let v = y as f32 / img_y as f32;
        let ray = ray::Ray::new(origin, lower_left_corner + (horizontal * u)
                                + (vertical * v));

        let color = color(ray);
        let ir = (255.99 * color.x()) as u8;
        let ig = (255.99 * color.y()) as u8;
        let ib = (255.99 * color.z()) as u8;

        *pixel = image::Rgb([ir, ig, ib]);
    }

    let path = Path::new("test.png");
    image::ImageRgb8(imgbuf).save(path).unwrap();
}
