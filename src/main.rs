use std::env;
use std::fs;
use std::io::Write;
use image::ImageReader;
use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    println!("Reading file: {}", file);
    let image = ImageReader::open(file)
        .expect("Should have been able to open the file")
        .decode()
        .expect("Should have been able to decode the image");

    let (width, height) = image.dimensions();
    println!("Image dimensions: {}x{}", width, height);
    let character_aspect_ratio_factor = 2.0;
    let target_width = 200;
    let aspect_ratio = target_width as f32 / width as f32;
    println!("Aspect ratio: {}", aspect_ratio);
    let width = 200;
    let height = (height as f32 * aspect_ratio / character_aspect_ratio_factor).round() as u32;
    let image = image.resize(width, height, image::imageops::FilterType::Nearest);
    println!("Resized image dimensions: {}x{}", width, height);

    let mut file = fs::File::create("output.txt").expect("Should have been able to create the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    println!("Final image dimensions: {}x{}", width, height);
    let characters = " `.'\"`^,:;_-~/!ilILtfjrcvunxzsyaeo17?+=*qpdbhk mwg345690OCGQUSPARDFJTZXYVNKEHBWM2&8%$#@";
    for y in 0..height {
        for x in 0..width {
            let pixel = rgb_image.get_pixel(x, y);
            let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
            let index = (brightness / 255.0 * (characters.len() - 1) as f32).round() as usize;
            let character = characters.chars().nth(index).unwrap();
            write!(file, "{}", character).expect("Should have been able to write to the file");
        }
        writeln!(file).expect("Should have been able to write to the file");
    }
}
