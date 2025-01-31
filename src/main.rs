use indicatif::{ProgressBar, ProgressStyle};
use pathtracer::Camera;
use pathtracer::Color;
use pathtracer::HitRecord;
use pathtracer::Hittable;
use pathtracer::HittableList;
use pathtracer::Interval;
use pathtracer::Point3;
use pathtracer::Ray;
use pathtracer::Sphere;
use pathtracer::Vec3;
use pathtracer::INFINITY;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
