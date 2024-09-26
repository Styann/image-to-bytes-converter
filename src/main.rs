use std::env;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::u8;
use std::fs::File;
use std::fs::OpenOptions;

use image::GenericImageView;
use image::Pixel;
use image::Rgb;
use image::imageops::FilterType;

use colored::Colorize;

fn rgb_to_16bits_rgb(rgb: Rgb<u8>) -> u16 {
    let r: u16 = (rgb.0[0] >> 3) as u16;
    let g: u16 = (rgb.0[1] >> 2) as u16;
    let b: u16 = (rgb.0[2] >> 3) as u16;

    return (r << 11 | g << 5 | b) as u16;
}

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let pwd_buf: PathBuf = env::current_dir()?;
    let pwd: String = pwd_buf.display().to_string();

    let input_path: PathBuf = Path::new(&pwd).join(&args[0]);
    let output_path: PathBuf = Path::new(&pwd).join("output.h");

    const SSD1331_WIDTH:u32 = 96;
    const SSD1331_HEIGHT:u32 = 64;

    if !input_path.exists() {
        panic!("{}", "this file doesn't exists !".red());
    }

    // open input and resize
    let input = image::open(input_path)
        .expect("File not found!")
        .resize(SSD1331_WIDTH, SSD1331_HEIGHT, FilterType::Triangle);

    // print warning if width or height does not fit
    if input.width() != SSD1331_WIDTH {
        let mut warn: String = String::from("resized width doesn't fit with ssd1331 width: ");
        warn.push_str(&format!("ssd1331 -> {} found -> {}", SSD1331_WIDTH, input.width()));
        println!("{}", warn.purple());
        // std::process::exit(0x0100);
    }

    if input.height() != SSD1331_HEIGHT {
        let mut warn: String = String::from("resized height doesn't fit with ssd1331 height: ");
        warn.push_str(&format!("ssd1331 -> {} found -> {}", SSD1331_HEIGHT, input.height()));
        println!("{}", warn.purple());
        // std::process::exit(0x0100);
    }

    let mut string_buf: String = String::from(format!("const uint16_t buffer[{}][{}] = {{\n", input.height(), input.width()));

    // fill buffer
    for x in 0..input.height() {
        let mut row_buf: String = String::from("\t{ ");

        for y in 0..input.width() {
            let pixel = input.get_pixel(y as u32, x as u32);
            // output_buf[x][y] = rgb_to_16bits_rgb(pixel.to_rgb());
            let new_rgb: String = format!("{:#06X}", rgb_to_16bits_rgb(pixel.to_rgb()));
            row_buf.push_str(&new_rgb);

            if y + 1 < input.width() { row_buf.push(',') }
            row_buf.push(' ');
        }

        row_buf.push('}');
        if x + 1 < input.height() { row_buf.push(',') }
        row_buf.push('\n');

        // finally add row to main buffer
        string_buf.push_str(&row_buf);
    }

    string_buf.push_str("};\n\n");
    // end fill buffer

    let mut output: File = OpenOptions::new().append(true).create(true).open(output_path)?;

    output.write_all(string_buf.as_bytes())?;

    // let mut output: File = File::create(output_path)?;
    // output.write_all(string_buf.as_bytes()).ok();

    // println!("{}", string_buf);

    Ok(())
}
