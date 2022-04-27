use std::num::Wrapping;
fn main() {
    //   变量
    //let x = 5;//不可变变量
    let mut x = 5;//可变变量
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    //   常量
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;//常量
    //常量与变量的区别：
    //声明常量使用const关键字而不是let，并且必须注明值的类型。
    //常量可以在任何作用域中声明，包括全局作用域。
    //常量只能被设置为常量表达式，而不可以是其他任何只能运行时计算出的值。
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
    //对常量的命名约定是在单词之间使用全大写加下划线，编译器能够在编译时计算一组有限的操作，这使我们可以选择以更容易理解和验证的方式写出此值，而不是将其设置为10,800。
    //在声明它的作用域之中，常量在整个程序生命周期中都有效，此属性使得常量可以作为多处代码使用的全局范围的值，例如一个游戏中所有玩家可以获取的最高分或者光速。
    //将遍布于程序中的硬编码值声明为常量，能帮助后来的代码维护人员了解值的意图。如果将来需要修改硬编码值，也只需要修改汇聚于一处的硬编码值。
    //   Shadowing（隐藏）
    //定义一个与之前同名的新变量，第一个变量被第二个隐藏了。
    let x2 = 9;
    let x2 = x2 + 1;
    {
        let x2 = x2 * 2;
        println!("The value of x2 in the inner scope is: {}", x2);
    }
    println!("The value of x2 is: {}", x2);
    /*
     The value of x2 in the inner scope is: 20
     The value of x2 is: 10
     */
    //隐藏与将变量标记为mut是有区别的。当没有使用let关键字就会导致编译时错误。使用let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
    //另一个区别是，当在此使用let时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。
    

    //      数据类型
    //在Rust中，每一个值都属于某一个数据类型，这告诉rust他被指定为何种数据，以便明确数据处理方式。我们将看到两类数据类型子集：标量(scalar)和符合（compound）。
    //rust是静态类型语言（在编译时就必须知道所有变量的类型）。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，比如: 
    //let guess: u32 = "42".parse().expect("not a number!");
    //这里字符串42可以转换为任意数值类型，编译器需要更多的信息， 来了解我们想要的类型。
    /*
      一、 标量类型   标量类型代表一个单独的值。rust有4种基本的标量类型：整型、浮点型、布尔类型和字符类型。



      1.整型
      rust内建的整数类型：
         长度   有符号   无符号
         8bit   i8       u8
         16bit  i16      u16
         32bit  i32      u32
         64bit  i64      u64
         128bit i128     u128
         arch   isize    usize
      可储存 -(2^(n-1)) ~ 2^(n-1) - 1
      isize和usize类型依赖运行程序的计算机架构：64位架构上它们是64位的，32位架构上它们是32位的。
      
      
      1.1数字字面值   可以是多种数字类型的数字字面值允许使用类型后缀，例如57u8来指定类型，同时也允许使用_作为分隔符，例如：1_000与1000相同。
         数字字面值                例子
         Decimal(十进制)          98_222
         Hex(十六进制)            0xff
         Octal(八进制)            0o77
         Binary(二进制)           0b1111_0000
         Byte(单字节字符)(仅限u8)  b'A'
      
      默认数字类型是i32。isize和usize主要作为某些集合的索引。
      
      整型溢出
      比方说有一个u8，它可以存放从零到255的值。那么当你将其修改为256时会发生什么呢？这被称为“整型溢出”（integer overflow），这会导致以下两种行为之一的发生。
      当在debug模式编译时，rust检查这类问题并使程序panic，这个术语被rust用来表明程序因错误而退出。
      在release构建中，rust不检测溢出，相反会进行一种被成为二进制补码包装（two's complement wrapping）的操作。简而言之，值256变成0,值257变成1，以此类推。
      依赖整型溢出被认为是一种错误，Wrapping。为了显式地处理溢出的可能性，你可以使用标准库在原生数值类型上提供的以下方法：
      1.所有模式下都可以使用wrapping_*方法进行包装，如wrapping_add
      2.如果check_*方法出现溢出，则返回None值
      3.overflowing_*方法返回值和一个布尔值，表示是否出现溢出
      4.用saturating_*方法在值的最小值或最大值处进行饱和处理
      
     */
     let mut a1: u8 = 255;
     // a1 += 1;//thread 'main' panicked at 'attempt to add with overflow', src\main.rs:78:6
     println!("{}", a1);
     let a2: u8 = 255;
     println!("{}", a2.wrapping_add(1));//0
     let aw = Wrapping(255u8);
     let awop1 = Wrapping(1u8);
     println!("{}", (aw + awop1).0);//0
     println!("{:?}", a2.checked_add(1));//None
     println!("value: {}, is overflow: {}", a2.overflowing_add(1).0, a2.overflowing_add(1).1);//value: 0, is overflow: true
     println!("value: {:?}", a2.saturating_add(1));//255
     /*
     2.浮点型
     rust也有两个原生的浮点数(floating-point number)类型。rust的浮点数类型是f32和f64，默认类型是f64。因为在现代cpu中，它与f32速度几乎一样，不过精度更高。所有浮点型都是有符号的。
     浮点数采用IEEE-754标准表示。f32是单精度浮点数，f64是双精度浮点数。
     */
     let x = 2.0;//f64
     let y: f32 = 3.0;//f32

     /*
     3.数值运算
     rust中的所有数字类型都支持基本数学运算：+,-,*,/,\//%。整数除法会向下舍入到最接近的整数。
     */
     let sum = 5 + 10;
     let difference = 95.5 - 4.3;
     let product = 4 * 30;
     let quotient = 56.7 / 32.2;
     let floored = 2 / 3;//0, 向下舍入到最接近的整数
     let remainder = 43 % 5;

     /*
     4.布尔型
     rust中布尔类型用bool表示。
     */
     let t = true;
     let f: bool = false;// 显示指定类型注解

     /*
     5.字符类型
     rust的char类型是语言中最原生的字母类型。
     注意，使用单引号声明char字面量，使用双引号声明字符串字面量。
     rust中char类型的大小为4个字节（four bytes），并代表了一个unicode标量值(unicode scalar value)，这意味着它可以比ASCII表示更多内容。在rust中，拼音字母(accented letters)，中文、日文、韩文等字符，emoji(绘文字)以及零长度的空白字符都是有效的char值。
     */
     let c = 'z';
     let z = 'Z';
     let heart_eyed_cat = '🐱';

     /*
     二、复合类型
     可以将多个值组合成一个类型。rust有两个原生的复合类型：元组(tuple)和数组(array)。

    1.元组类型
    元组是一个将多个其他类型的值组合进一个符合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
    我们使用包含圆括号中的逗号分隔的值列表来创建一个元组。元组中每一个位置都有一个类型，而这些不通知的类型也不必是相同的。这个例子中使用了可选的类型注解。
    tip变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值。
    程序首先创建一个元组并绑定到tup变量上。接着使用let和一个模式将tup分为3个不同的变量，x、y、z。这叫做解构（destructuring），因为它将一个元组拆成了3个部分。
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let ( x, y, z ) = tup;
    println!("The value of y is: {}", y);
    /*
    也可以使用点号.后跟值的索引来直接访问他们。
    */
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    /*
    没有任何值的元组()是一种特殊的类型，只有一个值，也写成()。该类型被称为单元类型(unit type)，而该值被称为单元值(unit value)。如果表达式不返回任何其他值，则会隐式返回单元值。
     */
     let x = (6);
     println!("{}", x);//6
    /*
    2.数组类型
    另一个包含多个值的方式是数组(array)。与元组不同，数组中的每个元素的类型必须相同。rust中的数组长度是固定的。
    我们将数组的值写在方括号内，用逗号分隔。
    当想要在栈(stack)而不是在堆(heap)上为数据分配空间，或者想要确保总是有固定数量的元素时，数组非常有用。
    但是数组并不如vector类型灵活。vector类型是标准库提供的一个允许增长和缩小长度的类似数组的集合类型。
    */
    let a = [1,2,3,4,5];
    //像这样编写数组的类型，在方括号中包含每个元素的类型后跟分号再后跟数组元素的数量。
    let a: [i32; 5] = [1,2,3,4,5];
    //i32: 元素类型, 5: 元素个数
    //你还可以通过在方括号中指定初始值再加分号再加元素个数的方式来创建一个每个元素都为相同值的数组：
    let a = [3; 5];//[3,3,3,3,3]
    //变量名为a的数组将包含5个
    //   访问数组元素
    //数组是可以在堆栈上分配的已知固定大小的单个内存块。可以使用索引来访问数组的元素。
    let a = [1,2,3,4,5];
    let first = a[0];//1
    let second = a[1];//2
    //程序在索引操作中使用一个无效的值（索引值超过数组长度）时导致运行时错误。程序带着错误信息退出，并且没有执行后面的语句。
    //当尝试用索引访问一个元素时，rust会检查指定的索引是否小于数组的长度。如果索引超出了数组长度，rust会panic。这种检查必须在运行时进行，因为编译器（当用户来提供访问数组的索引号）不知道用户在以后运行代码时将输入什么值。
    //这是第一个在实战中遇到的rust安全原则的例子。在很多底层语言中，并没有这类检查，这样当提供一个不正确的索引时，就会访问无效的内存。通过立即退出而不是允许内存访问并继续执行, rust让你避开此类错误。
    

    //函数
    //another_function定义在main函数之后也可以在之前。rust不关心函数定义于何处，只要定义了就行。
    //详见functions包下
    another_function();
}

//函数
fn another_function() {
    println!("Another function.");
}
