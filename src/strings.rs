pub(crate) fn string_example(){
    let s1: &str = "World"; // &str 是一个字符串切片的不可变引用。
    let mut s2: String = String::from("Hello ");//可变字符串
    s2.push_str(s1);//字符串拼接
    let s3: &str = &s2[6..]; //引用s2的字符串切片,并从index 6 开始截取到最后
    //在字符串中似乎不会受到引用的影响,但是在数组中就会受到影响
    println!("s1: {s1} s2: {s2} s3: {s3}");
    // 可以把 &str 当作 C++ 中的 const char*，但是它总是指向内存中的一个有效字符串
    // Rust 的 String 大致相当于 C++ 中 std::string, 与 C++ 不同，Rust 中的字符串切片不需要以空字符结尾，因为它们包含了长度信息。

}