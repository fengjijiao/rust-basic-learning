fn main() {
    //控制流
    //rust中最常见的用来控制执行流的结构是if表达式和循环
    //1.if表达式
    //if表达式中与条件关联的代码块有时被叫做arms，就像match表达式中的分支一样。
    //也可以包含一个可选的else表达式来提供一个在条件为假时应当执行的代码块。
    let number = 3;
    if number < 5 {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }
    //2.使用else if处理多重条件
    //使用过多的else if表达式会使代码显得杂乱无章，所以如果有多于一个else if表达式，最好重构代码(使用match)。
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
    //3.在let语句中使用if
    //因为if是一个表达式，我们可以在let语句右侧使用它。
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);//5
    //4.使用循环重复执行
    //多次执行同一段代码是很常见的，rust为此提供了多种循环(loops)。一个循环执行循环体中的代码直到结尾并紧接着回到开头继续执行。
    //rust有3种循环: loop、while和for。
    //4.1.使用loop重复执行代码
    //loop关键字告诉rust一遍又一遍的执行代码直到你明确要求停止。
    // loop {
    //     println!("again!");
    // }
    //4.2从循环返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);//20
    //4.3while条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    //4.4使用for遍历集合
    //可以使用while结构来遍历集合中的元素，比如数组。
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
    //更简洁的替代方案，可以使用for循环来对一个集合的每个元素执行一些代码
    let a = [10,20,30,40,50];
    for element in a {
        println!("The vaule is: {}", element);
    }
    //for range
    for number in (1..4).rev() {
        println!("{}!", number);//3!, 2!, 1!
    }
}
