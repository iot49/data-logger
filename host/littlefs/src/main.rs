// https://github.com/nickray/littlefs2

use littlefs2::io::prelude::*;
use littlefs2::{
    // consts,
    driver,
    fs::Filesystem,
    fs::File,
    io::Result,
    path::Path,
    ram_storage,
};

ram_storage!(tiny);

type Fs<'a> = Filesystem::<'a, RamStorage<'a>>;

fn w(fs: &Fs, path: &Path) {
    

    fs.create_file_and_then(path, |file| {
        file.write(b"abc\ndef\nghi")
    }).unwrap();
}

fn r(fs: &Fs, path: &Path) {
    let mut buf1 = [0u8; 5];
    let mut buf2 = [0u8; 7];
    fs.open_file_and_then(path, |file| {
        file.read(&mut buf1)
        // file.read(&mut buf2)
    }).unwrap();
    println!("read {:#?}", buf1);
    println!("read {:#?}", buf2);
}

fn main() {
    println!("little fs!");

    // example storage backend
    let mut ram = Ram::default();
    let mut storage = RamStorage::new(&mut ram);

    // must format before first mount
    Filesystem::format(&mut storage).unwrap();

    // must allocate state statically before use
    let mut alloc = Filesystem::allocate();
    let fs = Filesystem::mount(&mut alloc, &mut storage).unwrap();

    let path = Path::from_bytes_with_nul(b"file\x00").unwrap();

    //BoGUS:
    //let mut alloc = File::allocate();
    //let mut file = File::create("abc", &mut alloc, &mut storage, &mut fs);

    w(&fs, path);
    r(&fs, path);

    let mut buf = [0u8; 11];

    let name = b"example.txt\x00";
    let path = Path::from_bytes_with_nul(name).unwrap();
    fs.open_file_with_options_and_then(
        |options| options.read(true).write(true).create(true),
        &path,
        |file| {
            file.write(b"Why is black smoke coming out?!")?;
            file.seek(SeekFrom::End(-24)).unwrap();
            assert_eq!(file.read(&mut buf)?, 11);
            Ok(())
        }
    ).unwrap();
    assert_eq!(&buf, b"black smoke");
}
