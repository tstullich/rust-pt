use camera::Camera;
use hitable::HitRecord;
use hitable_list::HitableList;
use rand::{thread_rng, Rng};
use ray::Ray;
use vector::Vec3;

use std::f32;

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
        let num_samples: u16 = 32;
        let mut rng = thread_rng();
        let mut pixels: Vec<u8> = Vec::with_capacity(dim_y as usize * dim_x as usize * 3);
        for y in (0..dim_y).rev() {
            for x in 0..dim_x {
                let mut col = Vec3::new(0.0, 0.0, 0.0);

                // Sample a set number of times to determine color
                for _ in 0..num_samples {
                    let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (dim_x as f32);
                    let v = (y as f32 + rng.gen_range(0.0, 1.0)) / (dim_y as f32);

                    let ray = &self.camera.get_ray(u, v);
                    col = col + self.color(&ray, world, depth);
                }

                // Apply antialising by taking average of samples
                col = col / (num_samples as f32);

                // Apply some gamme correction
                col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

                let ir = (255.99 * col.x()) as u8;
                let ig = (255.99 * col.y()) as u8;
                let ib = (255.99 * col.z()) as u8;
                pixels.push(ir);
                pixels.push(ig);
                pixels.push(ib);
            }
        }
        pixels
    }

    // Computes the next ray based on the material that the Hitable object posseses
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
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
