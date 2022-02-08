
#![allow(unused_variables)]
#![allow(unused)]



use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s :{}, len is :{} ", s1, len);
    let s2 = String::from("Hello word!");
    let s2_first_word = first_word(&s2);
    println!("s2_first_word :{} ", s2_first_word);
    let user1 = User {
        active: false,
        user_id: 1,
        username: String::from("username"),
        nick_name: String::from("nickname"),
        email: String::from("email"),
    };

    let point0 = Point(0, 0, 0);
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    println!("The arae of the rectangle : {} ", rect1.ararea());

    println!("Coin: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    // if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => {
            println!("The max is :{} ", max)
        }
        _ => (),
    }
    // 等同于如下
    if let Some(max) = config_max {
        println!("Let if, the max is:{}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("Coin state from {:?}", state);
    } else {
        count += 1;
    }

    // panic
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 宏
    calculate! {
        eval 1 + 2
    }

    // 模块路径
    pub mod a {
        fn foo(){println!("a");}
        pub mod b {
            pub mod c {
                pub fn foo(){
                    // is a func
                    super::super::foo();
                    //slef::super::super::foo();
                }
            }
        }
    }

    a::b::c::foo();

    // 方法调用
    struct S;
    impl S {
        fn f() {println!("S");}
    }

    trait T1 {
        fn f() {println!("T1 func");}
    }

    impl T1 for S {}

    trait T2 {
        fn f() {
            println!("T2 func")
        }
    }
    impl T2 for S {}
    // 调用 inherent impl
    S::f();
    <S as T1>::f();
    <S as T2>::f();

    // 反省函数 turbofish 操作符
    Iterator::collect::<Vec<i32>>((0..10));
    
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        };
    }

    &s[..]
}

struct User {
    active: bool,
    user_id: u64,
    username: String,
    nick_name: String,
    email: String,
}

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

// 结构体方法

use std::f32::consts::E;

impl Rectangle {
    fn ararea(&self) -> i32 {
        self.width * self.height
    }
}

#[allow(dead_code)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?} .", state);
            25
        }
    }
}

// 泛型 看起来和 Java的类似
// fn largest<T> (list: &[T]) -> T {
//     let mut max = list[0];
//     for &item in list {
//         if item > max {
//             max = item;
//         }
//     }
//     max
// }


#[allow(unused)]
// rust 2018
#[macro_export]
macro_rules! calculate {
    (eval $e:expr) => {{
        let val: usize = $e;
        println!("{} = {}", stringify!{$e}, val)
    }};
}