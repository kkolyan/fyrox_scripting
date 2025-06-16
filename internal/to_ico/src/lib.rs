use std::fs;
use std::path::Path;

pub fn process_icon(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src_bytes = fs::read(src).unwrap();
    let src_image = ico::IconImage::read_png(src_bytes.as_slice()).unwrap();

    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    icon_dir.add_entry(ico::IconDirEntry::encode(&src_image).unwrap());

    let file = fs::File::create(dst).unwrap();
    icon_dir.write(file).unwrap();
}
