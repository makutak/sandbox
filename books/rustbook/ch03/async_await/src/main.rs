use futures::executor;

fn main() {
    executor::block_on(somothing_great_async_function());
}

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn somothing_great_async_function() -> i32 {
    let ans = async_add(2, 3).await;
    println!("{}", ans);
    ans
}
