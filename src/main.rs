use std::{io::Write, fs::File, num::ParseIntError, fs};
use image::{self, io::Reader, GenericImageView, imageops::FilterType};

fn list_filenames() -> Vec<String>{
    let mut filenames: Vec<String> = vec![];
    let mut i = 0;
    for file in fs::read_dir("./assets").unwrap(){
        filenames.insert(i, file.unwrap().path().display().to_string().replace("\\", "/").replace("./", ""));
        i+=1;
    }
    return filenames;
}

fn main() -> Result<(), ParseIntError>{
    const ASCII_CHARACTERS: [char; 11] = [' ', '.', '!', '|', '*', '#', '$', '6', '&', '%', '@'];
    const MAX_SIZE: (u32,u32) = (100,100);
    
    let mut txt_ascii = File::create("./ascii_image.txt").expect("Could not create the file");
    let filenames: Vec<String> = list_filenames();
    let mut ascii_image = String::new();

    for i in 0..filenames.len(){
        let image = Reader::open(format!("{}", filenames[i])).unwrap().with_guessed_format().unwrap().decode();
        let mut clone_img = image.unwrap().grayscale().clone();

        let res_x = if clone_img.dimensions().0 < MAX_SIZE.0 {clone_img.dimensions().0} else {MAX_SIZE.0};
        let res_y = if clone_img.dimensions().1 < MAX_SIZE.1 {clone_img.dimensions().1} else {MAX_SIZE.1};
        clone_img = clone_img.resize(res_x, res_y, FilterType::Nearest);

        let mut img_y = 0;
        for pixel in clone_img.pixels() {
            let coordinate_y = pixel.1;
            let color = pixel.2.0[0];

            if img_y != coordinate_y {
                ascii_image.push_str("\n");
                img_y = coordinate_y;
            }

            //selects ascii char by luminosity
            let character = ((color as f32 / 255.0) * (ASCII_CHARACTERS.len() - 1) as f32).round() as usize;
            ascii_image.push(ASCII_CHARACTERS[character]);
        };

        txt_ascii.write_all(ascii_image.as_bytes()).expect("Error");
        txt_ascii.write("\n\n\n\n\n\n\n".as_bytes()).expect("Error");
        ascii_image.clear();
    }
    Ok(())
}
