use rand::{thread_rng, Rng};
use ray::Ray;
use std::f32;
use vector::Vec3;

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    pub origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

/*
 * A simple implementation of a Camera. Will try and
 * add more documentation on this later
 */
impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vertical_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vertical_fov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;

        // Find orthonormal basis for camera directions
        let w = Vec3::unit_vec(lookfrom - lookat);
        let u = Vec3::unit_vec(vup.cross(&w));
        let v = w.cross(&u);

        let lower_left_corner = lookfrom
            - (u * half_width * focus_dist)
            - (v * half_height * focus_dist)
            - (w * focus_dist);
        let horizontal = u * half_width * focus_dist * 2.0;
        let vertical = v * half_height * focus_dist * 2.0;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            lens_radius,
        }
    }

    fn random_unit_in_disk(&self) -> Vec3 {
        let mut rng = thread_rng();
        let v1 = Vec3::new(1.0, 1.0, 0.0);
        let mut p = Vec3::new(
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
        );
        while p.dot(&p) >= 1.0 {
            p = (Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0) * 2.0) - v1;
        }
        p
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.random_unit_in_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let direction = self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
            - self.origin
            - offset;
        Ray::new(self.origin + offset, direction)
    }
}

#[test]
fn test_create() {
    let width = 1200;
    let height = 800;
    let lookfrom = Vec3::new(1.0, 2.0, 10.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        (width / height) as f32,
        0.1,
        dist_to_focus,
    );

    assert_eq!(
        cam.lower_left_corner,
        Vec3::new(-2.6787124, -2.692856, 0.80644226)
    );
    assert_eq!(cam.horizontal, Vec3::new(5.464072, -0.0, -0.5464072));
    assert_eq!(cam.vertical, Vec3::new(-0.10664777, 5.385712, -1.0664777));
    assert_eq!(cam.origin, Vec3::new(1.0, 2.0, 10.0));
    assert_eq!(cam.u, Vec3::new(0.9950372, -0.0, -0.09950372));
    assert_eq!(cam.v, Vec3::new(-0.019421138, 0.9807674, -0.19421138));
    assert_eq!(cam.lens_radius, 0.05);
}
