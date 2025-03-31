use pathtracer::Camera;
use pathtracer::HittableList;
use pathtracer::Lambertian;
use pathtracer::Material;
use pathtracer::Metal;
use pathtracer::Point3;
use pathtracer::Sphere;
use pathtracer::Vec3;
use std::rc::Rc;

fn main() {
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new_material(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new_material(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new_material(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new_material(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
