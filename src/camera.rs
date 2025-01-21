use std::fs::File;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    random_double, ray, unit_vector, vec3::random_on_hemisphere, write_color, Color, HitRecord,
    Hittable, Interval, Point3, Ray, Vec3, INFINITY,
};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: u32,       // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    image_height: u32,          // Rendered image height
    center: Point3,             // Camera center
    pixel00_loc: Point3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below
    pixel_samples_scale: f64,   // Color scale factor for a sum of pixel samples
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 100,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 1,
            pixel_samples_scale: 1.0,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let mut file = File::create("image.ppm").expect("Unable to create file");

        // Indicatif setup
        let total = self.image_height as u64;
        // Create a progress bar
        let pb = ProgressBar::new(total);

        // Customize the style
        pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)
            .expect("Unable to write header to file");

        for j in (0..self.image_height) {
            pb.inc(1);
            for i in (0..self.image_width) {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in (0..self.samples_per_pixel) {
                    let r = Self::get_ray(self, i, j);
                    pixel_color += Self::ray_color(self, &r, world);
                }
                write_color(&file, &(self.pixel_samples_scale * pixel_color));
            }
        }
        pb.finish_with_message("Done");
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64) / self.aspect_ratio).floor() as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        let center = Point3::new(0.0, 0.0, 0.0);

        // Determine the viewport dimensions.
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();

        if world.hit_interval(r, Interval::new(0.0, INFINITY), &mut rec) {
            let direction = random_on_hemisphere(&rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), world);
        }

        let unit_direction = unit_vector(r.direction());

        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i,j

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vec3 {
        return Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0);
    }
}
