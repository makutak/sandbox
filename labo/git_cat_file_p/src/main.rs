use libflate::zlib::Decoder;
use std::{
    fs::File,
    io::{self, Cursor, Read},
    str,
};

fn usage() {
    println!("Usage: ");
    println!("git_cat_file_p blob_file");
}

fn decompress(mut file: File) -> Result<String, io::Error> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut decoder = Decoder::new(Cursor::new(&buf)).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    unsafe {
        let str_data = String::from_utf8_unchecked(decoded_data);
        Ok(str_data)
    }
}

fn format(decoded: &str) -> &str {
    decoded.trim_end().split('\u{0}').collect::<Vec<_>>()[1]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        usage();
        std::process::exit(1)
    }

    let f =
        File::open("/home/tk/work/tmp/repo/.git/objects/a4/940ec3db4cd24542203a9447c4259c96294c09");
    //File::open("/home/tk/work/tmp/repo/.git/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad");
    //File::open("./tmp.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => panic!("{}: Can't open: {}", error, args[1]),
    };

    let decoded = decompress(file).unwrap();
    let formatted = format(&decoded);
    println!("{}", formatted);
}
