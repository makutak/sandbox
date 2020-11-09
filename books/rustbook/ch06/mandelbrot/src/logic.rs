fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
    // 複素数z_n の実部を xn, 虚部を yn とする。
    let mut xn = 0.0;
    let mut yn = 0.0;
    for i in 1..max_iter {
        let x_next = xn * xn - yn * yn + x0;
        let y_next = 2.0 * xn * yn + y0;
        xn = x_next;
        yn = y_next;

        if yn * yn + xn * xn > 4.0 {
            return i as u8; // 複素数の絶対値が2を超えると発散と判定する
        }
    }
    max_iter as u8
}
