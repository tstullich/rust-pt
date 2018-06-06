extern crate image;
extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod triangle;
mod vector;

use hitable::HitRecord;
use material::Material;
use rand::{thread_rng, Rng};
use ray::Ray;
use sphere::Sphere;
use triangle::Triangle;
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
fn color(r: &Ray, world: &hitable_list::HitableList, depth: u32) -> Vec3 {
    let hit_object = world.intersect(r, 0.001, std::f32::MAX);
    if hit_object.is_some() {
        // Retrieve intersected object properties
        let obj = hit_object.unwrap();
        // Compute where the next ray is going to bounce
        let scattered = compute_scatter_ray(&obj, r);
        // TODO Make the depth parameter adjustable
        if depth < 50 && scattered.is_some() {
            return color(&scattered.unwrap(), world, depth + 1)
                * obj.material.color();
        } else {
            // If we do not intercept anymore geometry we are finished
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = Vec3::unit_vec(r.direction());
    let t: f32 = (unit_direction.y() + 1.0) * 0.5;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Final output settings
    let dim_x: u32 = 1200;
    let dim_y: u32 = 800;

    // Camera setup
    // TODO Find a way to get around having to do this. Seems a bit hacky
    // Flipping the directions of the x and y coordinate since we write
    // into our output buffer starting from the top left corner of our
    // image and not from the bottom left
    let lookfrom = Vec3::new(4.0, 2.0, 10.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (dim_x / dim_y) as f32,
        aperture,
        dist_to_focus,
    );

    // Create our scene and add some geometry
    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        0.5,
        Material::Lambertian(Vec3::new(0.1, 0.2, 0.5)),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, 0.0),
        100.0,
        Material::Lambertian(Vec3::new(0.04, 0.67, 0.72)),
    )));

    world.add(Box::new(Triangle::new(
                Vec3::new(0.0, 0.0, 0.5),
                Vec3::new(0.0, 0.5, 0.5),
                Vec3::new(0.5, 0.0, 0.5),
                Material::Lambertian(Vec3::new(1.0, 0.0, 0.0)))));

    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, 0.0),
        0.5,
        Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.0),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, 0.0),
        0.5,
        Material::Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    )));

    // Options pertaining to the actual path tracing
    let depth: u32 = 0;
    let num_samples: u16 = 16;
    let mut rng = thread_rng();
    let mut imgbuf = image::ImageBuffer::new(dim_x, dim_y);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = vector::Vec3::new(0.0, 0.0, 0.0);
        println!("x: {}", x);
        println!("y: {}", y);

        // Sample a set number of times to determine color
        for _ in 0..num_samples {
            let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (dim_x as f32);
            let v = (y as f32 + rng.gen_range(0.0, 1.0)) / (dim_y as f32);

            let ray = &cam.get_ray(u, v);
            col = col + color(&ray, &world, depth);
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
