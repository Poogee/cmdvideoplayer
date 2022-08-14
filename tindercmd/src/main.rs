use std::collections::HashMap;


extern crate image;
use std::fs::File;
use std::io::prelude::*;
use std::env;


use crate::image::GenericImageView;
use image::DynamicImage;
use image::imageops::FilterType;


fn write_file(art: &str) {

    let mut file = File::create("art.txt").expect("Could not create the file");

    file.write_all(art.as_bytes()).expect("Error");
    println!("Art writen in: art.txt");
}

fn image_to_ascii(image: DynamicImage, resolution: u32) -> String{
    let pallete: [char; 7] = [' ', '.', '/', '*', '#', '$', '@'];
    let mut y = 0;
    let mut art = String::new();
    let small_img = image.resize(image.width() / resolution, image.height() / resolution, FilterType::Nearest);
    println!("Transforming image");
    for p in small_img.pixels() {
        if y != p.1 {
            art.push_str("\n");
            y = p.1;

        }
        
        let r = p.2.0[0] as  f32;
        let g = p.2.0[1] as f32;
        let b = p.2.0[2] as f32 ;
        //luminosidade
        let k = r * 0.3 + g * 0.59 + b * 0.11;
        let caracter = ((k / 255.0)  * (pallete.len() - 1) as f32).round() as usize;

        art.push(pallete[caracter]);
    }
    art
}



fn read_image(path: &str) -> DynamicImage {
    println!("Getting image data");
    let img = image::open(path).unwrap();
    

    return img

}


fn main() {
    handshake();
    let image = get_frame();
    println!("{}", image_to_ascii( ,5))
}
#[tokio::main]


async fn handshake() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:5000/")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(resp)
}

async fn get_frame() -> Result<DynamicImage , Box<dyn std::error::Error>>{
    let img_bytes = reqwest::get("http://127.0.0.1:5000/").await?.bytes().await?;
    let image = image::load_from_memory(&img_bytes)?;
    Ok(image)
}