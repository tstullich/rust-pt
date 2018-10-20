extern crate indicatif;
extern crate rand;
extern crate rayon;

use std::f32;

use camera::Camera;
use hitable::HitRecord;
use hitable_list::HitableList;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use vector::Vec3;

use self::indicatif::{ProgressBar, ProgressStyle};
use renderer::rayon::prelude::*;

pub struct Renderer {
    camera: Camera,
}

impl Renderer {
    pub fn new(camera: Camera) -> Renderer {
        Renderer { camera }
    }

    pub fn render(&self, dim_x: u32, dim_y: u32, world: &HitableList) -> Vec<u8> {
        // Options pertaining to the actual path tracing
        let depth: u32 = 0;
        let num_samples: u16 = 16;
        let progress_bar = &Box::new(ProgressBar::new(dim_x as u64 * dim_y as u64));
        progress_bar.set_message("Rendered Pixels");
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .progress_chars("##-"),
        );

        let pixels = (0..dim_y)
            .into_par_iter()
            .rev()
            .flat_map(|y| {
                (0..dim_x).into_par_iter().flat_map(move |x| {
                    let mut col = Vec3::new(0.0, 0.0, 0.0);

                    // Sample a set number of times to determine color
                    for _ in 0..num_samples {
                        let u = (x as f32 + rand::random::<f32>()) / (dim_x as f32);
                        let v = (y as f32 + rand::random::<f32>()) / (dim_y as f32);

                        let ray = &self.camera.get_ray(u, v);
                        col = col + self.color(&ray, world, depth);
                    }

                    // Apply antialising by taking average of samples
                    col = col / (num_samples as f32);

                    // Apply some gamme correction
                    col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

                    progress_bar.inc(1);
                    (0..3)
                        .into_par_iter()
                        .map(move |k| (255.99 * col[k as usize]).min(255.0) as u8)
                })
            })
            .collect();
        progress_bar.finish();

        pixels
    }

    // Computes the next ray based on the material that the Hitable object possesses
    // We could theoretically move this back into the color() function. I might
    // go ahead and do that later
    fn compute_scatter_ray(&self, intersected: &HitRecord, r: &Ray) -> Option<Ray> {
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
    fn color(&self, r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
        let hit_object = world.intersect(r, 0.001, f32::MAX);
        if hit_object.is_some() {
            // Retrieve intersected object properties
            let obj = hit_object.unwrap();
            // Compute where the next ray is going to bounce
            let scattered = self.compute_scatter_ray(&obj, r);
            // TODO Make the depth parameter adjustable
            if depth < 50 && scattered.is_some() {
                return self.color(&scattered.unwrap(), world, depth + 1) * obj.material.color();
            } else {
                // If we do not intercept anymore geometry we are finished
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = Vec3::unit_vec(r.direction());
        let t: f32 = (unit_direction.y() + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

#[test]
fn test_hit() {
    // Camera setup
    let width = 100;
    let height = 100;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        (width / height) as f32,
        0.0,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Add a single sphere
    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Vec3::new(0.5, 0.4, 0.5)),
    )));

    // See if the renderer runs
    let renderer = Renderer::new(cam);
    let pixels = renderer.render(width, height, &world);
    assert!(!pixels.is_empty());
}
