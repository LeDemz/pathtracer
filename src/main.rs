use pathtracer::Camera;
use pathtracer::Color;
use pathtracer::Dielectric;
use pathtracer::HittableList;
use pathtracer::Lambertian;
use pathtracer::Metal;
use pathtracer::Point3;
use pathtracer::Sphere;
use pathtracer::PI;

use std::rc::Rc;

fn main() {
    let R = f64::cos(PI / 4.0);

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.fvov = 90.0;

    cam.render(&world);
}
