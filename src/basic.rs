pub(crate) fn basic_example() {
    /*
    有符号整数        type = i8, i16, i32, i64, i128, isize	   == -10, 0, 1_000, 123_i64
    无符号整数        type = u8, u16, u32, u64, u128, usize	   == 0, 123, 10_u16
    浮点数	        type = f32, f64	                           == 3.14, -10.0e20, 2_f32
    字符串	        type = &str	                               == "foo", "two\nlines"
    Unicode标量类型	type = char	                               == 'a', 'α', '∞'
    布尔值	        type = bool	                               == true, false
    数组（Arrays）	type = [T; N]	                                   == [20, 30, 40], [0; 3]
    元组（Tuples）	type = (), (T,), (T1, T2), …	                   == (), ('x',), ('x', 1.2), …
     */
    切片();
    不可变长度数组();
    可变长度数组();
    元组();
    引用作用域();
    悬垂引用();
    循环();
}

//noinspection NonAsciiCharacters
fn 切片() {
    let mut a: [i8; 10] = [42; 10];// [type; size] == [42; 10] 第一个参数是数组的元素，第二个参数是数组的长度
    a[5] = 0;// 数组是通过size建立的,所以上面并不是两位长度的数组,而是10个42的数组,这里需要注意
    let mut s = &a[2..4]; // 切片,这里是一个引用,所以不会发生所有权的转移
    // let mut aa = *s; //doesn't have a size known at compile-time
    let mut aa = &*s;
    println!("aa: {:?}", aa);
    println!("a: {:?} s:{:#?}", a, s);
}

//noinspection NonAsciiCharacters
fn 不可变长度数组() {
    let mut b = [42, 13, 22, 44, 66, 11];// [type] 这里是传统的数组定义方式
    b[5] = 0;
    println!("b: {:?}", b);
}

//noinspection NonAsciiCharacters
fn 可变长度数组() {
    let mut b1 = Vec::new();
    b1.push(42);
    b1.push(13);
    b1.push(22);
    b1.push(44);
    b1.push(66);
    b1.push(11);
    // 上面的代码和下面的代码是等价的,但是下面的代码更加的简洁,通过vec卫生宏避免了写一大堆的push
    let mut b2 = vec![42, 13, 22, 44, 66, 11]; // 这里是向量的定义方式,向量是动态的数组,可以随意增加和减少
    b1[5] = 100;
    b2[5] = 100;
    println!("b1: {:?}", b1);
    println!("b2: {:#?}", b2);
}

//noinspection NonAsciiCharacters
fn 元组() {
    let mut t: (i8, bool, i32) = (7, true, 8);// 元组, 用逗号分隔
    t.1 = false;// 元组的访问方式
    println!("t: {:#?}", t);

    let (x, y, z) = t;// 元组的解构
    println!("x: {}, y: {}, z: {}", x, y, z);

    let void = (); // 空元组,可以理解为其他语言的void
    println!("void: {:#?}", void);
}

//noinspection NonAsciiCharacters
fn 引用作用域() {
    let mut x: i32 = 10;
    let immutable_x_before = x;
    let /*mut*/ ref_x: &mut i32 = &mut x; // 一个引用被认为是“借用（borrow）”了它指向的值。
    // 如果把let immutable_x_before = x;写在后面就违反了Rust的借用规则，因为不能在创建可变引用后再使用原始变量。
    // 需要等待解引用才能访问原始变量
    // 对于引用类型相当于是一个指针,所以这里如果使用 let mut,修改值的话就会直接修改指针指向的值
    let immutable_ref_x_before = *(&*ref_x);
    println!("引用修改前 ref_x: {immutable_ref_x_before}  x={immutable_x_before}");
    *ref_x = 20;
    /* let a = ref_x.count_ones();
     println!("a: {}", a);
     let immutable_x_after = *(&x);
     println!("immutable_x_after: {}", immutable_x_after);*/
    let immutable_ref_x_after = *ref_x; //解引用,否则后面的x变量是不允许使用的
    let immutable_x_after = *(&x);
    println!("引用修改后 ref_x: {immutable_ref_x_after}  x={immutable_x_after}");
    // Rust 有时会进行自动解引用。比如调用方法 ref_x.count_ones() 时，ref_x 会被解引用。
}

//noinspection NonAsciiCharacters
fn 悬垂引用() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        println!("x: {x} 悬垂引用不能使用");
        // ref_x = &x; //局部变量,与ref_x的生命周期并不是一样长的,跑完这个block,x就被回收了,所以会导致悬垂引用的错误
    }
    let x = 3;
    ref_x = &x;
    println!("ref_x: {ref_x}");
}

//noinspection NonAsciiCharacters
fn 循环() {
    let mut x = 6;  // 可变变量绑定
    // 未使用的值,推荐使用下划线前缀
    println!("开始啦");
    print!("{x} ");       // 与 printf 类似的输出宏
    // 谓词表达式不需要括号
    while x != 1 {       // 表达式周围没有括号
        if x % 2 == 0 {  // 与其他语言类似的数值计算
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
    println!("结束啦");
}