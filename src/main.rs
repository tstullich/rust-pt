extern crate png;
extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod renderer;
mod sphere;
mod triangle;
mod vector;

use material::Material;
use png::HasParameters;
use sphere::Sphere;
use triangle::Triangle;
use vector::Vec3;

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
        Material::Lambertian(Vec3::new(1.0, 0.0, 0.0)),
    )));

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

    let renderer = renderer::Renderer::new(cam);
    let pixels = renderer.render(dim_x, dim_y, &world);

    let path = std::path::Path::new("test.png");
    let file = std::fs::File::create(path).unwrap();
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, dim_x, dim_y);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}
