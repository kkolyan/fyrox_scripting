use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let src = args.get(1).unwrap();
    let dst = args.get(2).unwrap();

    let zip = ZipWriter::new(File::create_new(dst).unwrap());
    zip.create_from_directory(&PathBuf::from_str(src.as_str()).unwrap())
        .unwrap();
}
