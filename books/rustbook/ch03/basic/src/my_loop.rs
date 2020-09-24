pub fn exec() {
    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("sub loop end"); //ここまで来ないので表示されない
        }
        println!("main loop end"); //ここまで来ないので表示されない
    }
}
