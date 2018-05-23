extern crate image;

mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vector;

fn color(r: &ray::Ray, world: &hitable_list::HitableList) -> vector::Vec3 {
    let mut rec: hitable::HitRecord = hitable::HitRecord::new();
    if world.intersect(r, 0.0, std::f32::MAX, &mut rec) {
        return vector::Vec3::new(rec.normal.x() + 1.0,
                                 rec.normal.y() + 1.0,
                                 rec.normal.z() + 1.0) * 0.5
    }

    let unit_direction = vector::Vec3::unit_vec(r.direction());
    let t: f32 = (unit_direction.y() + 1.0) * 0.5;
    return vector::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) +
        vector::Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let lower_left_corner = vector::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vector::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vector::Vec3::new(0.0, 2.0, 0.0);
    let origin = vector::Vec3::new(0.0, 0.0, 0.0);

    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(sphere::Sphere::new(vector::Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(sphere::Sphere::new(vector::Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let img_x = 2000;
    let img_y = 1000;
    let mut imgbuf = image::ImageBuffer::new(img_x, img_y);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / img_x as f32;
        let v = y as f32 / img_y as f32;

        // TODO Check what goes wrong here
        let mut direction = lower_left_corner + horizontal * u + vertical * v;
        direction = vector::Vec3::new(direction.x() * -1.0, direction.y() * -1.0, direction.z());
        println!("{:?}", direction);
        let ray = ray::Ray::new(origin, direction);

        let color = color(&ray, &world);
        let ir = (255.99 * color.x()) as u8;
        let ig = (255.99 * color.y()) as u8;
        let ib = (255.99 * color.z()) as u8;

        *pixel = image::Rgb([ir, ig, ib]);
    }

    let path = std::path::Path::new("test.png");
    image::ImageRgb8(imgbuf).save(path).unwrap();
}
