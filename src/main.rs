extern crate image;
extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vector;

use hitable::HitRecord;
use material::Material;
use rand::{thread_rng, Rng};
use ray::Ray;
use sphere::Sphere;
use vector::Vec3;

// Computes the next ray based on the material that the Hitable object posseses
// We could theoretically move this back into the color() function. I might
// go ahead and do that later
fn compute_scatter_ray(intersected: &HitRecord, r: &Ray) -> Option<Ray> {
    intersected.material.scatter(r, &intersected)
}

/*
 * This is the function that currently does the most work for my path tracer.
 * The function calculates a color value by taking the initial ray passed in
 * through the main function, determines if any intersections have been made
 * with any of the geometry in the scene and then calculating a scattered
 * ray based on the type of material given. This process is recursively
 * executed until we have reached a finite number of bounces or we are
 * unable to intersect anymore geometry.
 */
fn color(r: &Ray, world: &hitable_list::HitableList, depth: u32, bounces: u8) -> Vec3 {
    if bounces > 0 {
        let hit_object = world.intersect(r, 0.001, std::f32::MAX);
        if hit_object.is_some() {
            // Retrieve intersected object properties
            let obj = hit_object.unwrap();
            // Compute where the next ray is going to bounce
            let scattered = compute_scatter_ray(&obj, r);
            // TODO Make the depth parameter adjustable
            if depth < 50 && scattered.is_some() {
                return color(&scattered.unwrap(), world, depth + 1, bounces - 1)
                    * obj.material.attenuation();
            } else {
                // If we do not intercept anymore geometry we are finished
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
    }

    let unit_direction = Vec3::unit_vec(r.direction());
    let t: f32 = (unit_direction.y() + 1.0) * 0.5;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Create our scene and add some geometry
    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian(Vec3::new(0.1, 0.2, 0.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(Vec3::new(0.8, 0.8, 0.0)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.3),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Material::Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    )));

    let cam = camera::Camera::new();
    let num_samples: u16 = 8;
    let mut rng = thread_rng();
    let dim_x: u32 = 1000;
    let dim_y: u32 = 500;
    let bounces: u8 = 16;
    let depth: u32 = 0;
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
            col = col + color(&ray, &world, depth, bounces);
        }

        // Apply antialising by taking average of samples
        col = col / (num_samples as f32);

        // Apply some gamme correction
        col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

        let ir = (255.99 * col.x()) as u8;
        let ig = (255.99 * col.y()) as u8;
        let ib = (255.99 * col.z()) as u8;

        *pixel = image::Rgb([ir, ig, ib]);
    }

    let path = std::path::Path::new("test.png");
    image::ImageRgb8(imgbuf).save(path).unwrap();
}
