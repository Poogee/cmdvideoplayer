use std::collections::HashMap;
use std::{thread, time};
extern crate ncurses;
use ncurses::*;

extern crate image;
use std::fs::File;
use rand::Rng;
use std::io::prelude::*;
use std::env;

use std::char;

use crate::image::GenericImageView;
use image::DynamicImage;
use image::imageops::FilterType;

fn remove_first<T>(vec: &mut Vec<T>) -> Option<T> {
    if vec.is_empty() {
        return None;
    }
    Some(vec.remove(0))
}

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
    // handshake();
    // let framecount = get_frame_count(0).unwrap();
    // for i in 1..framecount{
    //     println!("{}", image_to_ascii(get_frame(0, i).unwrap(), 3));
    //     let ten_millis = time::Duration::from_millis(100);
    //     thread::sleep(ten_millis);
    // }
      /* If your locale env is unicode, you should use `setlocale`. */
  // let locale_conf = LcCategory::all;
  // setlocale(locale_conf, "zh_CN.UTF-8"); // if your locale is like mine(zh_CN.UTF-8)

  /* Start ncurses. */
  initscr();
  start_color();
  init_pair(1,COLOR_WHITE, COLOR_BLUE);
  let mut args: Vec<String> = env::args().collect();
    remove_first(&mut args);
    if args.len() != 0 {
        if args.contains(&"--classic".to_string()){
            wbkgd(stdscr(), COLOR_PAIR(1));
        }
    }
  raw();

  /* Allow for extended keyboard (like F1). */
  keypad(stdscr(), true);
  noecho();

  /* Prompt for a character. */
  addstr("Do you want to watch a video ? [Y/n]");

  /* Wait for input. */
  let ch = getch();
  let normch =  char::from_u32(ch as u32).expect("Invalid char");
  let playing = true;
  if normch == 'y'
  {
    let xcoord = 60+20+20;
    let ycoord = 20;
    //let videonumber = rand::thread_rng().gen_range(0..1);
    while playing {
        let mut equal = 1;
        let videonumber = 1;
        let mut up = true;
        let framecount = get_frame_count(videonumber).unwrap();
      for i in 1..framecount{
          mvprintw(0,0,image_to_ascii(get_frame(videonumber, i).unwrap(), 6).as_str());
          mvprintw(ycoord-2,xcoord+2, "Subtitles:");
          //mvprintw(ycoord,xcoord-2, "Ne nado. Dyadya ne nado. Da kak eto kontrit'?");
          //mvprintw(ycoord-1,xcoord, "Ne nado dyadya. Neeeet");
          if equal == 1 {
              mvprintw(ycoord,xcoord, "[Music]");
              equal = equal + 1;
              up = true;
          } else if equal == 2 && up{
              mvprintw(ycoord,xcoord, "[mUsic]");
              equal = equal + 1;
          } else if equal == 3 && up {
              mvprintw(ycoord,xcoord, "[muSic]");
              equal = equal + 1;
          } else if equal == 4 && up {
              mvprintw(ycoord,xcoord, "[musIc]");
              equal = equal + 1; 
          } else if equal == 5 && up {
              mvprintw(ycoord,xcoord, "[musiC]");
              equal = equal - 1; 
              up = false; 
          }else if equal == 2 && !up{
              mvprintw(ycoord,xcoord, "[mUsic]");
              equal = equal - 1;
          } else if equal == 3 && !up {
              mvprintw(ycoord,xcoord, "[muSic]");
              equal = equal - 1;
          } else if equal == 4 && !up {
              mvprintw(ycoord,xcoord, "[musIc]");
              equal = equal - 1; 
          }

          // match equal{
          //     1 => mvprintw(0,20, "Музон"),
          //     2 => mvprintw(0,20, "мУзон"),
          //     3 => mvprintw(0,20, "муЗон"),
          //     4 => mvprintw(0,20, "музОн"),
          //     5 => mvprintw(0,20, "музоН"),
          //     _ => panic!()
          // };
           refresh();
  
          let ten_millis = time::Duration::from_millis(110);
          thread::sleep(ten_millis);
      }
      addstr("\nDo you want to watch the same video again [y/N]");
      refresh();
      let normresp = ' ';
        let mut resp = getch();
        let mut normresp = char::from_u32(resp as u32).expect("Invalid char");
        if normresp == 'ƚ'{
            resp = getch();
            normresp = char::from_u32(resp as u32).expect("Invalid char"); 
        }
        //dbg!(normresp);
      if normresp == 'n'{
        refresh();
        addstr("\nDo you want to watch the next video? [y/N]");
        let mut resp2 = getch();
        let mut normresp2 = char::from_u32(resp2 as u32).expect("Invalid char");
        if normresp2 == 'ƚ' {
            resp2 = getch();
            normresp2 = char::from_u32(resp2 as u32).expect("Invalid char");
        }
        //dbg!(normresp2);
        refresh();
        if normresp2 == 'n'{
            endwin();
        } else if normresp2 == 'y' {
            refresh();
            let videonumber = 1;//rand::thread_rng().gen_range(0..1);
            refresh();
            continue
        }
      } else if normresp == 'y' {
        continue  
      }
    }
  }
  else
  {
    endwin();
  }
  /* Allow for extended keyboard (like F1). */
  /*keypad(stdscr(), true);
  noecho();

  /* Prompt for a character. */
  addstr("Enter a character: ");

  /* Wait for input. */
  let ch = getch();
  if ch == KEY_F(1)
  {
    /* Enable attributes and output message. */
    attron(A_BOLD | A_BLINK);
    addstr("\nF1");
    attroff(A_BOLD | A_BLINK);
    addstr(" pressed");
  }
  else
  {
    /* Enable attributes and output message. */
    addstr("\nKey pressed: ");
    attron(A_BOLD | A_BLINK);
    addstr(format!("{}\n", char::from_u32(ch as u32).expect("Invalid char")).as_ref());
    attroff(A_BOLD | A_BLINK);
  }*/

  /* Refresh, showing the previous message. */
  refresh();
  /* Print to the back buffer. */
  //addstr("Hello, world!");
  getch();
  handshake();


  /* Print some unicode(Chinese) string. */
  // addstr("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

  /* Update the screen. */

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
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
