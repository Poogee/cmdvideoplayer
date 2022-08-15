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
    let framecount = get_frame_count(0).unwrap();
    for i in 1..framecount{
        while True{
        println!("{}", image_to_ascii(get_frame(0, i).unwrap(), 3));
    }
}
#[tokio::main]
async fn handshake() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:5000/")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[tokio::main]
async fn get_frame_count(videono : u32) -> Result<u32 , Box<dyn std::error::Error>>{
    let mut x = HashMap::new();
    x.insert("videono".to_string(), videono);
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:5000/getframecount").json(&x)
    .send()
    .await?.json::<HashMap<String, u32>>().await?;
    Ok(*res.get("framecount").unwrap())
    // let resp = reqwest::get("http://127.0.0.1:5000/")
    //     .await?
    //     .json::<HashMap<String, u32>>()
    //     .await?;
    // println!("{:#?}", resp);
    // Ok(())
}

#[tokio::main]
async fn get_frame(videono: u32, frameno: u32) -> Result<DynamicImage , Box<dyn std::error::Error>>{
    let mut x = HashMap::new();
    let client = reqwest::Client::new();
    x.insert("videono".to_string(), videono);
    x.insert("frameno".to_string(), frameno );
    //let img_bytes = reqwest::get("http://127.0.0.1:5000/getframe").await?.bytes().await?;
    let res = client.post("http://127.0.0.1:5000/getframe").json(&x)
    .send()
    .await?.bytes().await?;
    let image = image::load_from_memory(&res)?;
    Ok(image)
}
