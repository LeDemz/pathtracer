fn main(){

    // Image
    let image_width : u32 = 256;
    let image_height : u32 = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height{
        for i in 0..image_width{
        let r : f64 = (i as f64) / (image_width as f64 -1.0);
        let g : f64 = (j as f64) / (image_height as f64 -1.0);
        let b : f64 = 0.0;


        let ir : i32 = (255.999 *r).floor() as i32; 
        let ig : i32 = (255.999 *g).floor() as i32;
        let ib : i32 = (255.999 *b).floor() as i32;

        println!("{} {} {}", ir,ig,ib);
        }
    }

}