extern crate clap;
extern crate png;
extern crate rand;

mod aabb;
mod bvh_node;
mod camera;
mod hitable;
mod hitable_list;
mod material;
mod parser;
mod ray;
mod renderer;
mod sphere;
mod texture;
mod triangle;
mod vector;

use clap::{App, Arg};
use hitable_list::HitableList;
use material::Material;
use parser::Parser;
use png::HasParameters;
use rand::{thread_rng, Rng};
use sphere::{MovingSphere, Sphere};
use triangle::Triangle;
use vector::Vec3;

fn main() {
    // Parse command line arguments
    let matches = App::new("Raytracer")
        .version("0.1")
        .author("Tim Stullich")
        .about("A simple raytracer written in Rust")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("An OBJ format file to render")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("The pixel width of the output image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .help("The pixel height of the output image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("samples")
                .short("s")
                .long("samples")
                .value_name("SAMPLES")
                .help("The number of samples that will be taken for antialiasing")
                .takes_value(true),
        )
        .get_matches();

    // Final output settings
    let width = matches
        .value_of("width")
        .unwrap_or("1200")
        .parse::<u32>()
        .unwrap();
    let height = matches
        .value_of("height")
        .unwrap_or("800")
        .parse::<u32>()
        .unwrap();

    let world = if matches.value_of("file").is_some() {
        // Create our scene and add some geometry
        Parser::OBJ(String::from(matches.value_of("file").unwrap())).parse()
    } else {
        // No OBJ file provided. Create random world
        random_world()
    };

    // Camera setup
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = camera::Camera::new(
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

    let renderer = renderer::Renderer::new(cam);
    let pixels = renderer.render(width, height, &world);

    let path = std::path::Path::new("test.png");
    let file = std::fs::File::create(path).unwrap();
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}

fn random_world() -> HitableList {
    let mut rng = thread_rng();
    let mut world = hitable_list::HitableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Vec3::new(0.5, 0.4, 0.5)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let mat_type = rng.gen_range(0.0, 1.0);
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0, 1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat_type < 0.8 {
                    world.push(Box::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * rng.gen_range(0.0, 1.0), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Material::Lambertian(Vec3::new(
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                        )),
                    )));
                } else if mat_type < 0.95 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                                0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                                0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                            ),
                            0.5 * rng.gen_range(0.0, 1.0),
                        ),
                    )));
                } else {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
                    )));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.5, 0.0),
        0.5,
        Material::Lambertian(Vec3::new(0.1, 0.2, 0.5)),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(2.0, 0.5, 0.0),
        0.5,
        Material::Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 0.5, 0.0),
        0.5,
        Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    )));

    world
}
