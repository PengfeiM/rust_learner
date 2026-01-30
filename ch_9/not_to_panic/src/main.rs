use std::fs::File;
use std::io::{self, Read};

fn main() {
    // 调用处理Result的函数
    println!("开始执行程序...");

    // 调用try_result函数前的分隔符
    println!("----------------------------------------");
    // println!("正在执行try_result函数：演示match表达式处理Result");
    // try_result();
    // println!("try_result函数执行完成");
    // 调用try_result函数后的分隔符
    println!("----------------------------------------");

    // 调用matching_error函数前的分隔符
    println!("----------------------------------------");
    // println!("正在执行matching_error函数：演示错误类型的匹配处理");
    // matching_error();
    // println!("matching_error函数执行完成");
    // 调用matching_error函数后的分隔符
    println!("----------------------------------------");

    // 调用unwrap_result函数前的分隔符
    println!("----------------------------------------");
    // println!("正在执行unwrap_result函数：演示unwrap方法的使用");
    // unwrap_result();
    // println!("unwrap_result函数执行完成");
    // 调用unwrap_result函数后的分隔符
    println!("----------------------------------------");

    // 调用expect_result函数前的分隔符
    println!("----------------------------------------");
    // println!("正在执行expect_result函数：演示expect方法的使用");
    // expect_result();
    // println!("expect_result函数执行完成");
    // 调用expect_result函数后的分隔符
    println!("----------------------------------------");

    propagating_errors();

    println!("程序执行完毕.");
}

// 注释掉原来的try_result函数，因为它会导致panic
// /// 这个函数演示如何使用Result类型处理可能失败的操作
// /// 它尝试打开一个文件，如果文件不存在则panic
// /*
// fn try_result() {
//     println!("正在执行try_result函数：尝试打开'hello.txt'文件");
//
//     let greeting_file_result = File::open("hello.txt");
//
//     let greeting_file = match greeting_file_result {
//         Ok(file) => {
//             println!("成功打开文件！");
//             file
//         }
//         Err(error) => panic!("Failed to open the file, err: {error:?}"),
//     };
//
//     // 使用完文件后，显示一条消息
//     drop(greeting_file); // 显式释放文件资源
//     println!("文件操作完成，资源已释放");
// }
// */
// /// 这个函数演示如何优雅地处理文件操作中的错误
// /// 当文件不存在时，它会尝试创建该文件而不是直接panic
// /*
// fn matching_error() {
//     println!("正在执行matching_error函数：尝试打开或创建'hello.txt'文件");
//
//     let greeting_file_result = File::open("hello.txt");
//
//     let greeting_file = match greeting_file_result {
//         Ok(file) => {
//             println!("成功打开已存在的文件！");
//             file
//         }
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => {
//                     println!("文件不存在，已成功创建'hello.txt'文件！");
//                     fc
//                 }
//                 Err(e) => panic!("Failed to create the file, err: {e:?}"),
//             },
//             other_error => {
//                 panic!("Failed to open the file due to other error: {other_error:?}");
//             }
//         },
//     };
//
//     // 使用完文件后，显示一条消息
//     drop(greeting_file); // 显式释放文件资源
//     println!("文件操作完成，资源已释放");
// }
// */
/*
/// unwrap_result函数演示unwrap方法的使用
/// unwrap方法会在Result为Err时自动panic，并显示默认错误信息
/// 这种方式适合在确定操作会成功或不介意程序崩溃的情况下使用
fn unwrap_result() {
    println!("unwrap_result函数：尝试使用.unwrap()方法打开'hello.txt'文件");
    let greeting_file = File::open("hello.txt").unwrap();
    drop(greeting_file);
    println!("文件操作完成");
}*/

/*
/// expect_result函数演示expect方法的使用
/// expect方法与unwrap类似，但在panic时会显示我们自定义的错误信息
/// 这种方式比unwrap更友好，因为提供了更清晰的错误描述
fn expect_result() {
    println!("expect_result函数：尝试使用.expect()方法打开'hello.txt'文件");
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    drop(greeting_file);
    println!("文件操作完成");
}*/

fn propagating_errors() {
    let res = read_username_from_file();

    match res {
        Ok(username) => {
            println!("Success to get username: {username}")
        }
        Err(err) => {
            panic!("Inner func return an error: {err:?}")
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");
    //
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //
    // let mut username = String::new();
    //
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // a much simpler way to implement the logic above.
    // the operator: ?
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // or, it can even simpler.
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
