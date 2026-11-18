use std::io::Read;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let Some(apk_path) = args.get(1) else {
        println!("Usage: pif my_app.apk");
        std::process::exit(1);
    };

    let apk_reader = std::io::BufReader::new(std::fs::File::open(apk_path)?);
    let mut apk = zip::ZipArchive::new(apk_reader)?;

    for name in apk.file_names() {
        println!("In apk: {}", name?);
    }

    let mut shared_object_file = apk.by_name("lib/x86_64/libhello.so")?;
    let mut shared_object_buffer = Vec::new();
    let shared_object_size = shared_object_file.read_to_end(&mut shared_object_buffer)?;

    println!("Read {shared_object_size} bytes from .so");

    Ok(())
}
