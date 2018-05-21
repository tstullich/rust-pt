mod ray;
mod vector;

fn main() {
    let v1 = vector::Vec3::new(0.5, 0.3, 0.0);
    let v2 = vector::Vec3::new(0.0, 0.1, 0.0);
    let v3 = v1 + v2;
    println!("{:?}", v3);
}
