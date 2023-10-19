use std::any::Any;
use std::cell::Cell;
use std::fmt::{Debug, Display};


pub(crate) fn functionary_example() {
    // let mut rect = Rectangle { width: 10, height: 5 };//使用结构体方法直接创建,有点累
    let rect = Rectangle::new(1, 2);//通过添加的new方法创建,更加简洁
    rect.width.set(100);
    println!("rect: {:?} {:?} {:.2} {:#?}",
             rect.width.get(), rect.height.get(), rect.area()/*保留两位小数点*/, rect.inc_width(5));//不能在{}块中直接填写表达式,这里有点傻逼

    println!("文本: {}", pick_one("哈喽", "单车"));
    println!("数字: {}", pick_one(500, 250));
    let value:Box<dyn Any> = pick_two("单车",500, );
    let reference:&dyn Any = value.as_ref();
    let r = (&*reference);
    println!("动态类型 {:?}",r);
    match r {
        u32_value if u32_value.is::<u32>() => println!("value is u32 {:?}",r),
        str_value if str_value.is::<&str>() => println!("value is str {:?}",r),
        _ => println!("value is not u32 nor str, mother fucker"),
    }
}


fn pick_two< T:Copy + Debug + Display+ 'static, S:Copy + Debug + Display+ 'static>(a: T, b: S) -> Box<dyn Any> where T: Copy,  S: Copy{
    //通过进程标识符来判断奇偶性,然后返回不同的值
    if std::process::id() % 2 == 0 { Box::new(a) } else { Box::new(b) }
}



fn pick_one<T>(a: T, b: T) -> T {
    //通过进程标识符来判断奇偶性,然后返回不同的值
    if std::process::id() % 2 == 0 { a } else { b }
}



//定义一个结构体
struct Rectangle {
    width: Cell<u32>,
    height: Cell<u32>,
}

//实现结构体的方法
impl Rectangle {
    //定义静态方法new,创建一个新的结构体.Rust 没有自定义构造函数
    pub(crate) fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width: Cell::new(width), height: Cell::new(height) }
    }

    fn area(&self) -> u32 {
        let w = self.width.get();
        let h = self.height.get();
        w * h
    }

    fn inc_width(&self, delta: u32) -> u32 {
        let w = self.width.get();
        self.width.set(w+delta);
        return self.width.get();
    }
}