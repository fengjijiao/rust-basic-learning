fn main() {
    //1.参数
    //我们可以定义为拥有参数（parameters）的函数，参数是特殊变量，是函数签名的一部分。当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。技术上讲，这些具体值被称为参数(arguemnts)，但是在日常交流中，人们倾向于不区分使用parameter和argument来表示函数定义中的变量或者调用函数时传入的具体值。
    another_function!(32);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
