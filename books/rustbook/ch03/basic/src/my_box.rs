pub fn exec() {
    let byte_ary = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_ary));

    let byte_ary = [b'w', b'o', b'r', b'l', b'd', b'!'];
    print(Box::new(byte_ary));
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
