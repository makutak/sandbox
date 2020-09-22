fn func(code: i32) -> Result<i32, String> {
    println!("func!!!");
    println!("code: {}", code);
    Ok(100)
}

fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?; //エラーの場合はここで return result
    println!("error_handling!!!");
    println!("error_handling code: {}", code);
    Ok(100)
}

fn main() {
    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }

    let result: Result<i32, String> = Ok(200);

    if let Ok(code) = result {
        println!("code: {}", code);
    }

    println!("unwrap_or() \n");
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));

    println!("and_then() \n");
    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func); //func() は実行される
    println!("Ok next_resut: {:?}", next_result);
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func); //func() は実行されない
    println!("Err next_resut: {:?}", next_result);

    println!("? \n");
    let result: Result<i32, String> = Ok(200);
    let ret = error_handling(result);
    println!("Ok ret: {:?}", ret);
    let result: Result<i32, String> = Err("error".to_string());
    let ret = error_handling(result);
    println!("Err ret: {:?}", ret);
}
