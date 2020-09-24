trait Tweet {
    fn tweet(&self);

    fn tweet_twite(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uoooooooooooooooooohh!!!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!")
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

pub fn exec() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twite();
    dove.shout();

    let duck = Duck {};
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];

    println!("### bird vec ###");
    for bird in bird_vec {
        bird.tweet();
    }
}
