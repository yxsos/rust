/*
 * @Description: 20220508练习
 * @Author: Kevin Guan
 * @Email: allmemory@vip.qq.com
 * @Date: 2022-05-08 10:25:48
 * @LastEditTime: 2022-05-08 12:22:13
 * @LastEditors: your name
 * @FilePath: \rust\src\main.rs
 * 可以输入预定的版权声明、个性签名、空行等
 */

//struct Structure(i32);
// fn main() {
//     // //print 没有换行  println 有换行
//     // println!("Hello, world!");
//     // //eprint 输出到标准错误 ln 有换行
//     // eprintln!("error line");

//     // println!("{} days",31i64);

//     // println!("{} of {:b} people know binary, the other half don't", 1, 2);
//     // println!("{number:>0width$}", number=1,width=6);

//     // println!("My name is {0}, {1} {0}", "Bond","james");

//     // for i in 1..100 {
//     //     println!("工号为:ut{number:>0width$}",number=i,width=6);
//     // }
//     // println!("UT{number:>0width$}",number=426,width=6);

// }

// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8
// }

// fn main() {
//     let name = "Peter";
//     let age = 27;
//     let peter = Person { name, age };

//     // 美化打印
//     println!("{:#?}", peter);
// }

// 如果需要打印非标准库的内容有两种办法
// 办法一： 声明 #[derive(Debug)]  使用 fmt::Debug 方法
// 办法二： 重写 fmt::Display 实现  类似于C++重写输出函数

// struct Student{
//     name: String,
//     age:u8
// }

// // 推导 `Structure` 的 `fmt::Debug` 实现。
// // `Structure` 是一个包含单个 `i32` 的结构体。
// #[derive(Debug)]
// struct Structure(i32);

// // 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//     let name = String::from("张三");
//     let age = 18;
//     let student = Student{name,age};

//     println!("{0}{1}",student.name,student.age);

//     println!("{:?} months in a year.", 12);
//     println!("{1:?} {0:?} is the {actor:?} name.",
//              "Slater",
//              "Christian",
//              actor="actor's");

//     // `Structure` 也可以打印！
//     println!("Now {:?} will print!", Structure(3));

//     // 使用 `derive` 的一个问题是不能控制输出的形式。
//     // 假如我只想展示一个 `7` 怎么办？
//     println!("Now {:#?} will print!", Deep(Structure(7)));
// }

// 重写 fmt::display
// use std::fmt;

// struct Structure(i32);

// impl fmt::Display for Structure {
//     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
//         write!(f,"{}",self.0)
//     }
// }

// struct Student {
//     name: String,
//     age: u8
// }

// impl fmt::Display for Student {
//     fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
//         write!(f,"我叫{0},今年:{1}岁",self.name,self.age)
//     }
// }

// fn main() {
//     let student = Student{name:"张三".to_string(),age:18};
//     println!("{}",student);
// }

// 复习 fmt::display重写
// 导入 std::fmt模块

use std::fmt;

struct Point {
    x: f64,
    y: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"{}+{}i",self.x,self.y)
    }
}

struct Complex {
    x:f64,
    y:f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"Complex:real:{},imag:{}",self.x,self.y)
    }
}

fn main (){
    let point = Point{x:3.3,y:7.2};
    println!("Display:{}",point);

    let complex = Complex{x:3.3,y:7.2};
    println!("{}",complex);
}
