use std::io::{self,Write};
use std::fs::File;
use indicatif::{ProgressBar, ProgressStyle};
fn main(){

    // Image
    let image_width : u32 = 256;
    let image_height : u32 = 256;

    let mut file = File::create("image.ppm").expect("Unable to create file");

    // Indicatif setup
    let total = image_height as u64;

    // Create a progress bar
    let pb = ProgressBar::new(total);

    // Customize the style
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("#>-"),
    );



    // Render
    writeln!(file,"P3\n{} {}\n255", image_width, image_height).expect("Unable to write header to file");
    for j in 0..image_height{
        pb.inc(1);
        for i in 0..image_width{
        let r : f64 = (i as f64) / (image_width as f64 -1.0);
        let g : f64 = (j as f64) / (image_height as f64 -1.0);
        let b : f64 = 0.0;


        let ir : i32 = (255.999 *r).floor() as i32; 
        let ig : i32 = (255.999 *g).floor() as i32;
        let ib : i32 = (255.999 *b).floor() as i32;

        writeln!(file,"{} {} {}", ir,ig,ib).expect("Unable to write content to file");
        }
    }
    pb.finish_with_message("Done");

}