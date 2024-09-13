use inquire::{Confirm, MultiSelect, Text};
use rand::Rng;

fn main() {
    //let mut 為可更改
    //Rust 世界多數都是不可變的
    
    let length_ans = Text::new("請問您的密碼長度要設定多少？").prompt();
    let mut length:i32 = 8;

    match length_ans {
        Ok(length_ans) => match length_ans.parse::<i32>() {
            Ok(number) => {
                println!("設定的密碼長度為: {}", number);
                length = number;
            }
            Err(_) => {
                println!("您輸入的不是有效的整數");
            }
        },
        Err(_) => {
            println!("請輸入密碼長度");
        }
    }
    
    let mut charset = String::from("");
    const PASSWORD_NUMBER: &str = "1234567890";
    let contain_number = Confirm::new("密碼是否包含數字").with_default(true).prompt();

    match contain_number {
        Ok(contain_number) => {
            if contain_number {
                println!("你的密碼將包含數字");
                charset.push_str(PASSWORD_NUMBER)
            } else {
                println!("你的密碼不包含數字");
            }
        }
        Err(_) => println!("請選擇是否包含數字"),
    }


    const PASSWORD_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const PASSWORD_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const PASSWORD_SYMBOL: &str = "!@#$%^&*()_+-=[]{}|;:',.<>?";


    let options = vec!["lowercase", "uppercase", "symbol"];
    let char_ans = MultiSelect::new("請選擇密碼是否要包含:", options).prompt();

    match char_ans {
        Ok(choices) => {
            for choice in choices {
                match choice {
                    "lowercase" => charset.push_str(PASSWORD_LOWERCASE),
                    "uppercase" => charset.push_str(PASSWORD_UPPERCASE),
                    "symbol" => charset.push_str(PASSWORD_SYMBOL),
                    _ => (),
                }
            }
        }
        Err(_) => println!("選擇出現錯誤"),
   }

    let mut password = generate_password(&charset, length); 

    match contain_number {
        Ok(contain) => {
            if contain {
                loop {
                    let origin_result = generate_password(&charset, length);
                    if origin_result.chars().any(|a| a.is_digit(10)) {
                        password = origin_result;
                        break;
                    }
                }
            }
        }
        Err(_) => {}
    }  
    println!("您的密碼為：{:?}", password)
}


fn generate_password(charset: &str, length: i32) -> String {
    let mut rng = rand::thread_rng();
    let char_bytes: &[u8] = charset.as_bytes();

    let result = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..char_bytes.len());
            char_bytes[idx] as char
        })
        .collect();

    result
}