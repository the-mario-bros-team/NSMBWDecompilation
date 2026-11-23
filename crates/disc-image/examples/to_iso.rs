//! Writes a disc image out as a plain ISO, re-encrypted (for testing the
//! crate with an ISO made from another format):
//! `cargo run --release -p disc-image --example to_iso -- <image> <out.iso>`.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [from, to] = args.as_slice() else {
        eprintln!("usage: to_iso <image> <out.iso>");
        std::process::exit(2);
    };
    let options = nod::OpenOptions { rebuild_encryption: true, ..Default::default() };
    let mut disc = nod::Disc::new_with_options(from, &options).expect("open the image");
    let mut out = std::io::BufWriter::new(std::fs::File::create(to).expect("create the ISO"));
    let n = std::io::copy(&mut disc, &mut out).expect("write the ISO");
    println!("{n} bytes");
}
