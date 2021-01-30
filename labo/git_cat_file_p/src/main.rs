use libflate::zlib::Decoder;
use std::{
    error::Error,
    fs::File,
    io::{Cursor, Read},
    str,
};

fn usage() {
    println!("Usage: ");
    println!("git_cat_file_p blob_file");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        usage();
        std::process::exit(1)
    }

    let mut file =
        File::open("/home/tk/work/tmp/repo/.git/objects/a4/940ec3db4cd24542203a9447c4259c96294c09")
            .unwrap();
    // File::open("/home/tk/work/tmp/repo/.git/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad")
    //     .unwrap();

    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();
    //println!("{:?}", buf);
    let mut decoder = Decoder::new(Cursor::new(&buf)).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    let str_data = str::from_utf8(&decoded_data).unwrap();
    println!("{}", str_data);
    // println!(
    //     "{}",
    //     str_data.trim_end().split('\u{0}').collect::<Vec<_>>()[1]
    // );

    Ok(())
}
