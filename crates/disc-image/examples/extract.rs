//! Extracts a disc image to a folder, for testing the crate:
//! `cargo run --release -p disc-image --example extract -- <image> <folder>`.

use std::sync::atomic::AtomicBool;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [image, folder] = args.as_slice() else {
        eprintln!("usage: extract <image> <folder>");
        std::process::exit(2);
    };
    let img = match disc_image::Image::open(std::path::Path::new(image)) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    println!("{} {}, revision {}: {}", img.format(), img.id(), img.revision(), img.title());
    let started = Instant::now();
    let mut last = 0;
    let r = img.extract(std::path::Path::new(folder), &mut |done, total| {
        let pct = done * 100 / total.max(1);
        if pct >= last + 10 {
            last = pct;
            println!("{pct}%");
        }
    }, &AtomicBool::new(false));
    match r {
        Ok(n) => println!("{n} files in {:.1} s", started.elapsed().as_secs_f32()),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
