use pathtracer::Camera;
use pathtracer::Color;
use pathtracer::Dielectric;
use pathtracer::HittableList;
use pathtracer::Lambertian;
use pathtracer::Metal;
use pathtracer::Point3;
use pathtracer::Sphere;
use pathtracer::Vec3;
use pathtracer::PI;

use std::rc::Rc;

fn main() {
    let R = f64::cos(PI / 4.0);

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.fvov = 90.0;

    cam.lookfrom = Point3::new(-2.0,2.0,1.0);
    cam.lookat   = Point3::new(0.0,0.0,-1.0);
    cam.vup      = Vec3::new(0.0,1.0,0.0);


    cam.render(&world);
}
