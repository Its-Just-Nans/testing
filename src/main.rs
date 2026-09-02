fn main() {
    use std::fs::File;
    use zip::ZipArchive;

    let mut archive = ZipArchive::new(File::open("luau-windows.zip").unwrap()).unwrap();
    archive.extract("out_dir").unwrap();
}
