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

pub fn support_impliment_macro() {
    let mut p = HappyPersion {
        name: "Takashi".to_string(),
        state: Emotion::Happy,
    };

    println!("{}", p.get_happy());

    p.state = Emotion::Anger;
    //println!("{}", p.get_anger());

    let x = 1;
    println!("{}", f(x));
    let x = 0;
    println!("{}", f(x));
    let x = 2;
}

enum Emotion {
    Anger,
    Happy,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPersion {
    name: String,
    state: Emotion,
}

impl Emotional for HappyPersion {
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }

    fn get_happy(&mut self) -> String {
        format!("{} is always happy.", self.name)
    }

    fn tell_state(&self) -> String {
        todo!()
    }
}

fn f(x: usize) -> &'static str {
    match x {
        n if n * n % 3 == 0 => "3n",
        n if n * n % 3 == 1 => "3n+1 or 3n+2",
        _ => unreachable!(), // コンパイラは上記条件で網羅していることを判定できないため必要
    }
}
