use futures::executor;
use std::future::Future;

fn main() {
    executor::block_on(somothing_great_async_function());
    executor::block_on(something_great_async_function_2());
    executor::block_on(something_great_async_function_3());
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

// async ではないので下記はコンパイルエラーとなる。
/*
fn something_wrong_function() -> i32 {
    let ans = async_add(2, 3).await;
    ans
}
 */
