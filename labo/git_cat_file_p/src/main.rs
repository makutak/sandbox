use libflate::zlib::Decoder;
use std::{
    error::Error,
    fs::File,
    io::{Cursor, Read},
    str,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("../.git/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad").unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();
    println!("{:?}", buf);

    let mut decoder = Decoder::new(Cursor::new(&buf)).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();
    println!("{:?}", str::from_utf8(&decoded_data).unwrap());

    Ok(())
}
