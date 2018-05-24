extern crate image;
extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vector;

use rand::{thread_rng, Rng};

fn random_unit_in_sphere() -> vector::Vec3 {
    let mut rng = thread_rng();
    let mut p = vector::Vec3::new(rng.gen_range(0.0, 1.0),
                                  rng.gen_range(0.0, 1.0),
                                  rng.gen_range(0.0, 1.0));

    while p.squared_length() >= 1.0 {
        let rand_x: f32 = rng.gen_range(0.0, 1.0);
        let rand_y: f32 = rng.gen_range(0.0, 1.0);
        let rand_z: f32 = rng.gen_range(0.0, 1.0);
        p = vector::Vec3::new(rand_x, rand_y, rand_z) * 2.0;
        p = p - vector::Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color(r: &ray::Ray, world: &hitable_list::HitableList, bounces: u8) -> vector::Vec3 {
    let mut rec: hitable::HitRecord = hitable::HitRecord::new();
    // Going to restrict the number of times a ray can bounce for now
    // otherwise we run into stack overflow problems
    if bounces > 0 && world.intersect(r, 0.001, std::f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_unit_in_sphere();
        let ray = ray::Ray::new(rec.p, target - rec.p);
        return color(&ray, world, bounces - 1) * 0.5;
    }

    let unit_direction = vector::Vec3::unit_vec(r.direction());
    let t: f32 = (unit_direction.y() + 1.0) * 0.5;
    return vector::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) +
        vector::Vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(sphere::Sphere::new(vector::Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(sphere::Sphere::new(vector::Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = camera::Camera::new();
    let num_samples = 4;
    let mut rng = thread_rng();
    let dim_x: u32 = 2000;
    let dim_y: u32 = 1000;
    let bounces: u8 = 4;
    let mut imgbuf = image::ImageBuffer::new(dim_x, dim_y);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = vector::Vec3::new(0.0, 0.0, 0.0);

        // Sample a set number of times to determine color
        for _ in 0..num_samples {
            let r_x: f32 = rng.gen_range(0.0, 1.0);
            let r_y: f32 = rng.gen_range(0.0, 1.0);
            let u = (x as f32 + r_x) / dim_x as f32;
            let v = (y as f32 + r_y) / dim_y as f32;

            let ray = &cam.get_ray(u, v);
            col = col + color(&ray, &world, bounces);
        }

        // Apply antialising by taking average of samples
        col = col / (num_samples as f32);

        // Apply some gamme correction
        col = vector::Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

        let ir = (255.99 * col.x()) as u8;
        let ig = (255.99 * col.y()) as u8;
        let ib = (255.99 * col.z()) as u8;

        *pixel = image::Rgb([ir, ig, ib]);
    }

    let path = std::path::Path::new("test.png");
    image::ImageRgb8(imgbuf).save(path).unwrap();
}
