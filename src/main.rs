use image::{self, DynamicImage, GenericImageView, ImageBuffer};
use std::{io, process};
use colored::Colorize;

fn main() {
  println!("---------- {} ----------", "Welcome to Parasitic Image".blue());
  println!("{}", "Condition: ratio of both host image and parasitic image must be same.\n".red());

  let (host, parasite, result) = paths();

  println!("\n------------------------------------------------");

  let (host, parasite) = match open_images(host, parasite) {
    Ok(value) => value,
    Err(()) => process::exit(0)
  };

  process(host, parasite, result);
  println!("{}", "Finished!".green());
  
}

fn process(host: DynamicImage, parasite: DynamicImage, result: String) {
  /* parasite thumbnail size change */
  let (hostx, hosty) = host.dimensions();
  let (parax, paray) = parasite.dimensions();
  let thumbnail_option: bool;

  let parasite = if hostx as f64 / hosty as f64 > parax as f64 / paray as f64 {
    thumbnail_option = true;
    parasite.thumbnail(hostx, paray * parax / hostx)
  } else {
    thumbnail_option = false;
    parasite.thumbnail(parax * paray / hosty, hosty)
  };

  /* parasitize the host */
  let (hostx, hosty) = host.dimensions();
  let (parax, paray) = parasite.dimensions();

  let draw = |x, y| {
    let rgba = host.get_pixel(x, y);
    if rgba[3] == 0 {
      image::Rgba::<u8>([0, 0, 0, 0])
    } else {
      if thumbnail_option {
        parasite.get_pixel(x, y + (paray - hosty) / 2)
      } else {
        parasite.get_pixel(x + (parax - hostx) / 2, y)
      }
    }
  };
  
  let result_image = ImageBuffer::from_fn(hostx, hosty, draw);
  result_image.save(result).unwrap();
}

fn open_images(host: String, parasite: String) -> Result<(DynamicImage, DynamicImage), ()> {
  let mut failed_count = 0;

  let host = match image::open(host) {
    Ok(value) => value,
    Err(_) => {
      println!("{}", "Failed to open host image.".red());
      failed_count += 1;
      empty_image()
    }
  };

  let parasite = match image::open(parasite) {
    Ok(value) => value,
    Err(_) => {
      println!("{}", "Failed to open parasite image.".red());
      failed_count += 1;
      empty_image()
    }
  };

  if failed_count == 0 {
    println!("{}", "Completed loading host and parasitic images.".green());
    return Result::Ok((host, parasite));
  }

  return Err(());
}

fn empty_image() -> DynamicImage {
  DynamicImage::new_luma16(0, 0)
}

fn paths() -> (String, String, String) {
  let host_image_path = input(&format!("Enter the path to the {} image:", "host".purple()));
  println!();
  let parasitic_image_path = input(&format!("Enter the path to the {} image:", "parasitic".purple()));
  println!();
  let result_path = input(&format!("Enter the path to the {} image:", "result".purple()));

  return (host_image_path, parasitic_image_path, result_path);
}

fn input(output: &str) -> String {
  println!("{}", output);
  let mut scan = String::new();
  io::stdin().read_line(&mut scan).expect("Failed to get input");
  return String::from(scan.trim());
}
