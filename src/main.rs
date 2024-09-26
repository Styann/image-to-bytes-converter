use std::env;
// use std::fmt::format;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::u8;
use std::fs::File;
use std::fs::OpenOptions;

use image::DynamicImage;
use image::GenericImageView;
use image::Pixel;
use image::{Rgb, Rgba};
use clap::Parser;

#[derive(Parser)]
struct Args {
    image_path: String,

    // #[clap(short, long)]
    // output_path: String
}

fn rgb_to_16bits_rgb(rgb: Rgb<u8>) -> u16 {
    let r: u16 = (rgb.0[0] >> 3) as u16;
    let g: u16 = (rgb.0[1] >> 2) as u16;
    let b: u16 = (rgb.0[2] >> 3) as u16;

    return (r << 11 | g << 5 | b) as u16;
}

fn parse_image(image: &DynamicImage, buffer: &mut String, tab: String) {
    buffer.push_str(format!("{}{}", tab, "{\n").as_str());

    for x in 0..image.height() {
        let mut row_buf: String = String::from(format!("{}    {} ", tab, "{"));

        for y in 0..image.width() {
            let pixel: Rgba<u8> = image.get_pixel(y as u32, x as u32);
            let new_rgb: String = format!("{:#06X}", rgb_to_16bits_rgb(pixel.to_rgb()));
            row_buf.push_str(&new_rgb);

            if y + 1 < image.width() {
                row_buf.push(',')
            }

            row_buf.push(' ');
        }

        row_buf.push('}');

        if x + 1 < image.height() {
            row_buf.push(',')
        }

        row_buf.push('\n');

        // finally add row to main buffer
        buffer.push_str(&row_buf);
    }

    buffer.push_str(format!("{}{}", tab, "}").as_str());
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();
    let pwd: PathBuf = env::current_dir()?;

    let mut height: u32 = 0;
    let mut width: u32 = 0;

    let mut last_image_i: u32 = 1;

    loop {
        let image_path: PathBuf = PathBuf::from(args.image_path.replace("%d", &last_image_i.to_string()));

        if !image_path.exists() {
            break;
        }

        last_image_i += 1;
    }

    let mut string_buf: String = String::from("const uint16_t buffer[%n][%h][%w] = {\n");

    for i in 1..last_image_i {
        let image_path: PathBuf = PathBuf::from(args.image_path.replace("%d", &i.to_string()));

        if !image_path.exists() {
            break;
        }

        print!("{} ", image_path.to_string_lossy());

        let image: DynamicImage = image::open(image_path).expect("File not found!");

        if i == 1 {
            height = image.height();
            width = image.width();
        }

        println!("width: {} height: {}", image.width(), image.height());

        parse_image(&image, &mut string_buf, String::from("    "));

        if (i + 1) < last_image_i {
            string_buf.push(',');
        }

        string_buf.push('\n');
    }

    string_buf.push_str("};\n\n");

    string_buf = string_buf
        .replace("%n", &last_image_i.to_string())
        .replace("%h", &height.to_string())
        .replace("%w", &width.to_string());

    // writing string_buf in a file
    let output_path: PathBuf = Path::new(&pwd).join("output.h");
    println!("writing output into {}", output_path.to_string_lossy());

    let mut output: File = OpenOptions::new().append(true).create(true).open(output_path)?;

    output.write_all(string_buf.as_bytes())?;

    Ok(())
}
