use std::io::Write;

pub fn exec() {
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
