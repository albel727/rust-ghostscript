extern crate clap;
extern crate env_logger;
extern crate ghostscript as gs;
extern crate image;
#[macro_use]
extern crate log;

mod grabber;

use clap::{App, Arg};
use grabber::PageGrabberDisplayCallback;
use gs::callback::display::DisplayFormat as DF;
use std::path::Path;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn file_to_images(input_file: &str, grayscale: bool, resolution: u32, start: u32, num: Option<u32>) -> Vec<grabber::RawImage> {
    let mut builder = gs::builder::GhostscriptBuilder::new();
    let mut my_callback = PageGrabberDisplayCallback::new();

    builder.with_default_device_list(Some(&["display"]));
    builder.with_display(true);

    let mut display_format = DF::ALPHA_NONE | DF::DEPTH_8 | DF::BIG_ENDIAN | DF::TOP_FIRST;
    if grayscale {
        display_format |= DF::COLORS_GRAY;
    } else {
        display_format |= DF::COLORS_RGB;
    }
    let display_format = display_format.format_as_init_arg();
    debug!("DisplayFormat: {}", display_format);

    let resolution = format!("-r{}", resolution);
    let first_page = format!("-dFirstPage={}", start);
    let last_page = num.map(|num| format!("-dLastPage={}", start + num - 1));

    let mut init_params = vec![
        "-sDEVICE=display",
        display_format.as_ref(),
        "-dNOPAUSE",
        "-dSAFER",
        &resolution,
        "-dTextAlphaBits=4",
        "-dGraphicsAlphaBits=4",
        &first_page,
    ];

    if let Some(last_page) = last_page.as_ref() {
        init_params.push(last_page);
    }

    init_params.push("--");
    init_params.push(input_file);

    builder.with_init_params(&init_params);

    builder
        .build(&mut my_callback)
        .has_quit()
        .expect("Interpreter failed to start or kept running");

    my_callback.into_pages()
}

fn is_natural_int(val: String) -> Result<(), String> {
    let val: u32 = val.parse().map_err(|_| String::from("Invalid number"))?;
    if val < 1 {
        return Err(String::from("The number must be greater than 0"));
    }
    Ok(())
}

fn main() {
    let mut builder = env_logger::LogBuilder::new();
    builder.parse("trace");
    builder.target(env_logger::LogTarget::Stdout);
    builder.init().unwrap();

    let matches = App::new("Ghostscript PS/PDF-to-image converter")
        .version(VERSION.unwrap_or("unknown"))
        .author("Alex Belykh <albel727@ngs.ru>")
        .about("Converts a range of pages in PS or PDF document into images of specified format and resolution.")
        .arg(
            Arg::with_name("grayscale")
                .help("Disables colors")
                .short("g")
                .long("gray"),
        )
        .arg(
            Arg::with_name("force")
                .help("Allows overwriting existing files")
                .short("f")
                .long("force"),
        )
        .arg(
            Arg::with_name("dpi")
                .help("Sets output resolution [default: 300]")
                .takes_value(true)
                .short("d")
                .long("dpi")
                .validator(is_natural_int),
        )
        .arg(
            Arg::with_name("start")
                .help("Sets the number of the first page to output [default: 1]")
                .takes_value(true)
                .short("s")
                .long("start")
                //.default_value("1") //FIXME: clap help display for this is buggy (issue 1140)
                .validator(is_natural_int),
        )
        .arg(
            Arg::with_name("num")
                .help("Sets the number of pages to output (all by default)")
                .takes_value(true)
                .short("n")
                .long("num")
                .validator(is_natural_int),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT_DIR")
                .help("Sets the output directory to use")
                //.required(true)
                .index(2)
                .default_value("output"),
        )
        .get_matches();

    let file_name = matches.value_of("INPUT").unwrap();
    let output_dir = Path::new(matches.value_of("OUTPUT_DIR").unwrap_or("output"));
    let grayscale = matches.is_present("grayscale");
    let force = matches.is_present("force");
    let dpi: u32 = matches
        .value_of("dpi")
        .map(|n| n.parse().unwrap())
        .unwrap_or(300);
    let start: u32 = matches
        .value_of("start")
        .map(|n| n.parse().unwrap())
        .unwrap_or(1);
    let num: Option<u32> = matches.value_of("num").map(|n| n.parse().unwrap());

    let imgs = file_to_images(file_name, grayscale, dpi, start, num);

    std::fs::create_dir_all(output_dir).expect("Unable to create output directory");

    for (idx, img) in imgs.iter().enumerate() {
        let file_name: std::path::PathBuf = output_dir.join(format!("page_{}.png", idx + start as usize));

        let depth_bits = gs::callback::display::depth_bits(img.format).expect("Format of unknown depth");
        let image_type = match img.format & DF::MASK_COLORS {
            DF::COLORS_RGB => image::RGB(depth_bits),
            DF::COLORS_GRAY => image::Gray(depth_bits),
            _ => panic!("Unexpected image format: {:?}", img.format),
        };

        if force {
            std::fs::remove_file(&file_name).ok();
        } else if file_name.exists() {
            panic!("File already exists: {:?}", file_name);
        }

        image::save_buffer(&file_name, &img.data, img.width, img.height, image_type).expect("Failed to save image");
    }
}
