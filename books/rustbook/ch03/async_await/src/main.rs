use futures::executor;
use std::future::Future;

fn main() {
    executor::block_on(somothing_great_async_function());
    executor::block_on(something_great_async_function_2());
    executor::block_on(something_great_async_function_3());
    executor::block_on(caluculate());
    executor::block_on(move_to_async_block());
}

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn somothing_great_async_function() -> i32 {
    let ans = async_add(2, 3).await;
    println!("{}", ans);
    ans
}

/*
async fn は impl Future の糖衣構文になっているため、
下記のように Future を直接記述しても同じである。
しかし普段は処理の見通しが良くなるなどのメリットがあるため、
async/.await が使わることが多い。
 */
fn something_great_async_function_2() -> impl Future<Output = i32> {
    async {
        let ans = async_add(2, 3).await;
        println!("{}", ans);
        ans
    }
}

async fn something_great_async_function_3() -> i32 {
    let ans1 = async_add(2, 3).await;
    let ans2 = async_add(3, 4).await;
    let ans3 = async_add(4, 5).await;
    let result = ans1 + ans2 + ans3;
    println!("{}", result);
    result
}

async fn print_result(value: i32) {
    println!("value: {}", value)
}

async fn caluculate() -> i32 {
    let add1 = async_add(2, 3).await;
    // これはy評価されない
    print_result(add1);

    let add2 = async_add(3, 4).await;
    // これは評価される
    print_result(add2).await;
    async_add(add1, add2).await
}

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_valiable = "this is outside".to_string();
    // 通常ならここで、outside_variable の所有権がなくなるのでコンパイルは通らないが、
    async move {
        // move を用いると、変数の所有権を async ブロックの中に移し、ブロック内でも使用できる。
        println!("{}", outside_valiable);
    }
}

// async ではないので下記はコンパイルエラーとなる。
/*
fn something_wrong_function() -> i32 {
    let ans = async_add(2, 3).await;
    ans
}
 */
