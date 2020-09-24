use std::io::Write;

pub fn output_macro() {
    print!("hello");
    println!("hello {}", "world");
    eprint!("hello {}", "error");
    eprintln!("hello");

    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    dbg!(w);

    let mut w = Vec::new();
    writeln!(&mut w, " is ABC");
    dbg!(w);
}

pub fn access_resource_macro() {
    println!("defined in file: {}", file!());
    println!("defined in line: {}", line!());
    println!("is test: {}", cfg!(unitx));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));
}
