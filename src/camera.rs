use ray::Ray;
use vector::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let mut direction = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        // TODO Find a way to get around having to do this. Seems a bit hacky
        // Flipping the directions of the x and y coordinate since we write
        // into our output buffer starting from the top left corner of our
        // image and not from the bottom left
        let x = direction.x();
        let y = direction.y();
        direction.set_x(x * -1.0);
        direction.set_y(y * -1.0);
        Ray::new(self.origin, direction)
    }
}
