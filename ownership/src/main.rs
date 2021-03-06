//所有权
//它让rust无需垃圾回收（garbage collector）即可保障内存安全，因此理解rust中所有权如何工作是十分重要的。
//所有权相关功能：借用(borrowing)、slice以及rust如何在内存中布局数据。

/*
rust的核心功能（之一）是所有权。
所有程序都必须管理其运行时使用计算机内存的方式。一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存；在另一些语言中，程序员必须亲自分配和释放内存。
rust则选择了第3种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规则，程序都不能编译。
在运行时，所有权系统的任何功能都不会减慢程序。

栈和堆的区别

栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同。
1.栈中所有数据都必须占用已知且固定的大小。在编译时大小未知或大小可能变化的数据，要改为存储在堆上。
2.堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。内存分配器(memory allocator)在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的指针(pointer)。这个过程称作在堆上分配内存(allocating on the heap)，有时简称为“分配”（allocating）。将数据推入栈中并不被认为是分配。因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。
3.入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。相比之下，在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。
4.访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。
5.处理器在处理的数据彼此较接近的时候（比如在栈上）比 较远的时候（比如可能在堆上）能更好的工作。在堆上分配大量的空间也可能消耗时间。

当你的代码调用一个函数时，传递给函数的值（包含可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移除栈。

跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。
一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的主要目的就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。


所有权规则

1.rust中的每一个值都有一个被称为其所有者(owner)的变量。
2.值在任一时刻有且只有一个所有者。
3.当所有者(变量)离开作用域，这个值将被丢弃。



变量作用域
{//s在这里无效,它尚未声明
    let s = "hello";//从此处起，s是有效的
    //使用s
}//此时作用域已结束，s不再有效

字符串字面量是被硬编码进程序里的字符串值，字符串字面量是很方便的，不过它们并不适合使用文本的每一种场景。原因之一就是它们是不可变的。另一个原因是并非所有字符串的值都能在编写代码时就知道：
例如，想要获取用户输入并存储该值该怎么办？
为此，rust有第二个字符串类型，String。
这个类型管理被分配在堆上的数据，所以能够存储在编译时未知大小的文本。
可以使用from函数基于字符串字面量来创建String，如下：
let s = String::from("hello");
这2个冒号::是运算符，允许将特定的from函数置于String类型的命名空间(namespace)下，而不需要使用类似string_from这样的名字。在方法语法(method syntax)部分会着重讲解这个语法而且在第七章的路径用于引用模块树中的项中会讲到模块的命名空间。

可以修改此类字符串：
let mut s = String::from("hello");
s.push_str(", world!");//push_str()在字符串后追加字面量
println!("{}", s);//将打印`hello, world!`

那么这里有什么区别呢？为什么String可变而字面量却不行呢？区别在于两个类型对内存的处理上。

内存与分配
就字符串字面量来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面量快速且高效。不过这些特性都只得益于字符串字面量的不可变性。
不幸的是，我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。

对于String类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
1.必须在运行时向内存分配器(memory allocator)请求内存。
2.需要一个当我们处理完String时将内存返回给分配器的方法。

第一部分由我们完成：当调用String::from时，它的实现(implementation)请求其所需的内存。这在编程语言中是非常通用的。

然而，第二部分实现起来就各有区别了。在有垃圾回收(garbage collector, GC)的语言中，GC记录并清除不再使用的内存，而我们并不需要关心它。在大部分没有GC的语言中，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的使用一样。从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。如果忘记回收了会浪费内存。如果过早回收了，将会出现无效变量。如果重复回收，这也是个bug。我们需要精确的为一个allocate配对一个free。

rust采用了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。
{
    let s = String.from("hello");//从此处起，s是有效的
    //使用s
}//此作用域已结束
//s不再有效

这是一个将String需要的内存返回给分配器的很自然的位置；当s离开作用域的时候。当变量离开作用域，rust为我们调用了一个特殊的函数。这个函数叫做drop，在这里String的作者可以放置释放内存的代码。rust在结尾的}处自动调用drop。
注意：在c++中,这种item在生命周期结束时释放资源的模式有时被称作资源获取即初始化(resource acquisition is initialization(RAII))。如果你使用过RAII模式的话应该对Rust的drop函数并不陌生。

这个模式对编写Rust代码的方式有着深远的影响。现在它看起来很简单，不过在更复杂的场景下代码的行为可能是不可预测的，比如当有多个变量使用在堆上分配的内存时。


变量与数据交互的方式（一）：移动
在Rust中，多个变量可以采取不同的方式与同一数据进行交互。
let x = 5;
let y = x;
因为整数是有已知固定大小的简单值，所以这2个5都被放入了栈中。
let s1 = String::from("hello");
let s2 = s1;
String由三部分组成：一个指向存放字符串内容的指针，一个长度和一个容量。这一组数据存储在栈上。
长度表示String的内容当前使用了多少字节的内存。容量是String从分配器总共获取了多少字节的内存。
当我们将s1赋值给s2，String的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据。
如果rust也拷贝了堆上的数据，那么操作s2=s1在堆上数据比较大的时候会对运行时性能造成非常大的影响。
之前我们提到过当变量离开作用域后，Rust自动调用drop函数并清理变量的堆内存。当rust不复制堆数据只复制栈数据时，2个数据指针指向了同一位置。这就有了一个问题：当s2和s1离开作用域，他们都会尝试释放相同的内存。这是一个叫做二次释放(double free)的错误，也是之前提到过的内存安全性bug之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
为了确保内存安全，在let s2 = s1之后，Rust认为s1不再有效，因此Rust不需要在s1离开作用域后清理任何东西。看看在s2被创建之后尝试使用s1会发生什么;
let s1 = String::from("hello");
let s2 = s1;

println("{}, world!", s1);
这段代码会报错，因为rust禁止你使用无效引用。
如果你在其他语言中听说过术语浅拷贝(shallow copy)和深拷贝(deep copy)，那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。不过因为rust同时使第一个变量无效了，这个操作被称为移动(move)，而不是浅拷贝。

这样就解决了我们的问题！因为只有s2是有效的，当期离开作用域，他就释放自己的内存。

另外，这里还隐含了一个设计选择：Rust永远也不会自动创建数据的“深拷贝”。因此，任何自动的复制可以被认为对运行时性能影响较小。

变量与数据交互的方式（二）：克隆
如果我们确实需要深度复制String中对上的数据，而不仅仅是栈上的数据，可以使用一个叫做clone的通用函数。
let s1 = String::from("hello");
let s2 = s1.clone();
println("s1 = {}, s2 = {}", s1, s2);

这段代码能正常运行，这里堆上的数据确实被复制了。
当出现clone调用时，你知道一些特定的代码被指向而且这些代码可能相当消耗资源。你很容易察觉到一些不寻常的事情正在发生。

只在栈上的数据：拷贝
let x = 5;
let y = x;

像整型这样的在编译时已知大小的类型被整个存储到栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量y后使x无效。换句话说，这里没有深浅拷贝的区别，所以这里调用clone并不会与通常的浅拷贝有什么不同，我们可以不用管它。

Rust有一个叫做Copy trait的特殊注解，可以用在类似整型这样的存储在栈上的类型上。如果一个类型实现了Copy trait,那么一个旧的变量在将其赋值给其他变量后仍然可用。Rust不允许自身或者任何部分实现了Drop trait的类型使用Copy trait。如果我们对其值离开作用域时需要特殊处理的类型使用Copy注解，将会出现一个编译时错误。
那么哪些类型实现了Copy trait呢？
不过作为一个通用规则，任何一组简单标量值的组合都可以实现Copy，任何不需要分配内存或某种形式资源的类型都可以实现Copy。如下是一些Copy的类型：
所有整数类型，如u32。
布尔类型，bool，它的值是true和false;
所有浮点数类型，比如f64。
字符类型，char。
元组，当且仅当包含的类型也都实现Copy的时候。比如，(i32, i32)实现了Copy，但(i32, String)就没有。


所有权与函数
将值传递给函数在语义上与给变量赋值相似。像函数传递值可能会移动或者赋值，就像赋值语句一样。
fn main() {
    let s = String::from("hello");//s进入作用域
    takes_ownership(s);//s的值移动到函数里
    let x = 5;//x进入作用域
    makes_copy(x);//x应该移动函数里，但i32是Copy的，所以后面可继续使用x

}//这里, x先移出了作用域，然后是s.但因为s的值已被移走，
//没有特殊之处

fn takes_ownership(some_string: String) {//some_string进入作用域
    println!("{}", some_string)
}//这里,some_string移出作用域并调用drop方法。
//占用内存被释放

fn makes_copy(some_integer: i32) {//some_integer 进入作用域
    println!("{}",some_integer)
}//这里,some_integer 移出作用域，没有特殊之处

当尝试在调用takes_ownership后使用s时，Rust会抛出一个编译时错误。这些静态检查使我们免于犯错。

返回值与作用域

返回值也可以转移所有权。
{
    lets1 = givens_ownership();//gives_ownership将返回值转移给s1
    let s2 = String::from("hello");//s2进入作用域
    let s3 = takes_and_gives_back(s2);//s2被移动到takes_and_gives_back中，它也将返回值移给s3
}//这里,s3移出作用域并被丢弃。s2也移出作用域，但已被移走，所以什么也不会发生。s1离开作用域并被丢弃

fn gives_ownership() -> String {//gives_ownership会将返回值移动给调用它的函数
    let some_string = String::from("yours");//some_string进入作用域
    some_string//返回some_string并移出给调用的函数
}

//takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {//a_string进入作用域
    a_string//返回a_string并移出给调用的函数
}

变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过drop被清理掉，除非数据被移动为另一个变量所有。
虽然这样是可以的，但是在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可以想返回函数体中产生的一些数据。

我们可以使用元组来返回多个值:

let s1 = String::from("hello");
let (s2, len) = calculate_length(s1);
println!("The length of '{}' is {}.", s2, len);

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();//len()返回字符串的长度
    (s, length)
}
但是这未免有些形式主义，而且这种场景应该很常见。幸运的是，Rust对此提供了一个不用获取所有权就可以使用值的功能，叫做引用(references)。



引用与借用

我们必须将String返回给调用函数，以便在调用calculate_length后仍能使用String，因为String被移动到了calculate_length内。相反我们可以提供一个String值的引用（reference）。引用(reference)像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值。
let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("The length of '{}' is {}.", s1, len);

fn calculate_length(s: &String) -> usize {
    s.len()
}
首先，注意变量声明和函数返回值中的所有元组代码都消失了。其次，注意我们传递&s1给calculate_length，同时在函数定义中，我们获取&String而不是String。这些&符号就是引用，它们允许你使用值但不获取其所有权。

&String s指向String s1

注意：与使用&引用相反的操作是解引用(deferencing)，它使用解引用运算符，*。


仔细看看这个函数调用：
let s1 = String::from("hello");
let len = calculate_length(&s1);
&s1语法让我们创建一个指向值s1的引用，但是并不拥有它。因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。
同理，函数签名&来表明参数s的类型是一个引用。
fn calculate_length(s: &String) -> usize {//s is a reference to a String
    s.len()
}//这里s 离开了作用域，但因为它并不拥有引用值的所有权，所以什么也不会发生

变量s有效的作用域与函数参数的作用域一样，不过当s停止时并不丢弃引用指向的数据，因为s并没有所有权。当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。
我们将创建一个引用的行为称为借用(borriwing)。正如现实生活中，如果一个人拥有某样东西，你可以从他哪里借用。当你使用完毕，必须还回去，我们并不拥有它。

如果我们尝试修改借用的变量，这将报错(正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。)

可变引用
我们通过一个小调整就能修复上面的错误，允许我们修改一个借用的值，这就是可变引用（mutable reference）:
let mut s = String::from("hello");
change(&mut s);

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

首先，我们必须将s改为mut。然后在调用change函数的地方创建一个可变引用&mut s，并更新函数签名以接受一个可变引用some_string: &mut String。这就非常清楚的表明，change函数将改变它所借用的值。
可变引用有一个很大的限制：在同一时间只能有一个对某一特定数据的可变引用。这些尝试创建两个s的可变引用的代码会失效。

let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);
//error

这个报错说这段代码是无效的，因为我们不能在同一时间多次将s作为可变变量借用。第一个可变的借入在r1中，并且必须持续在println!中使用它，但是在那个可变引用的创建和它的使用之间，我们又尝试在r2中创建另一个可变引用，该引用借用与r1相同的数据。
防止同一时间对同一数据进行多个可变引用的限制允许可变性，不过是以一种受限制不过是以一种受限制的方式允许。新Rustacean们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。这个限制的好处是Rust可以在编译时就避免数据竞争。数据竞争（data race）类似于竟态条件，它可由这3个行为造成：1.两个或更多指针同时访问统一数据。2.至少有一个指针被用来写入数据。3，没有同步数据访问的机制。
数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能同时拥有：
let mut s = String::from("hello");
{
    let r1 = &mut s;
}//r1在这里离开了作用域所以我们完全可以创建一个新的引用
let r2 = &mut s;

rust在同时使用可变与不可变引用时也采用的类似的规则。
let mut s = String::from("hello");
let r1 = &s;//没问题
let r2 = &s;//没问题
let r3 = &mut s;//大问题
println!("{}, {}, and {}", r1, r2, r3);
我们也不能在拥有不可变引用的同时拥有可变引用。不可变引用的用户可不希望在它们的眼皮底下值就被意外的改变了！然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。例如，因为最后一次使用不可变引用（println!）。发生在声明可变引用之前，所以如下代码是可以编译的：
let mut s = String::from("hello");

let r1 = &s;//没问题
let r2 = &s;//没问题
println!("{} and {}", r1, r2);
//此位置之后r1和r2不再使用

let r3 = &mut s;//没问题
println!("{}", r3);

不可变引用r1和r2的作用域在println!最后一次使用之前结束，这也是创建可变引用r3的地方。它们的作用域没有重叠，所以代码是可以编译的。编译器在作用域结束之前判断不再使用的引用的能力被称为非词法作用域生命周期(Non-Lexical Lifetimes，简称NLL)。

尽管这些错误有时使人沮丧，但请牢记这是Rust编译器在提前指出一个潜在的bug（在编译时而不是在运行时）并精确显示问题所在。这样你就不必去跟踪为何数据并不是你想象中的那样。


悬垂引用(Dangling References)
在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针(dangling pointer)，所谓悬垂指针是指其指向的内存可能已经被分配给其他持有者。相比之下，Rust中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
让我们尝试创建一个悬垂引用
let reference_to_nothing = dangle();
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
因为s是在dangle函数内创建的，当dangle的代码执行完毕后，s将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的String，这可不对。
这里的解决方法是直接返回String: 
fn dangle() -> String {
    let s = String::from("hello");
    s
}
//这就没有任何错误了。所有权被移动出去，所以没有值被释放。




引用的规则
1.在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
2.引用必须总是有效的。



Slice 类型


slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice是一类引用，所以他没有所有权。

编程习题： 编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
view first_word(...)

first_word函数有一个参数&String。因为我们不需要所有权，所以这没问题。不过应该返回什么呢？我们并没有一个真正获取部分字符串的办法。不过，我们可以返回单词结尾的索引，结尾由一个空格表示。

因为需要逐个进行元素的检查String中的值是否为空格，需要用as_bytes方法将String转化为字节数组： let bytes = s.as_bytes();
接下来，使用iter方法在字节数组上创建一个迭代器：
for (i, &item) in bytes.iter().enumerate() {
    //...
}

iter方法返回集合中的每一个元素，而enumerate包装了iter的结果，将这些元素作为元组的一部分来返回。enumerate返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。这比我们自己计算索引要方便一些。
因为enumerate方法返回一个元组，我们可以使用模式...(i, &item)..来解构。所以在for循环中，我们指定了一个模式，其中元组中的i是索引而元组中&item是单个字节。因为我们从.iter().enumerate()中获取了集合元素的引用，所以模式中使用了&。

在for循环中，我们通过字节的字面值语法来寻找空格的字节。如果找到了一个空格，返回它的位置。否则，使用s.len()返回字符串的长度：
if item == b' ' {
    return i;
}
s.len()
现在有了一个找到字符串中第一个单词结尾索引的方法，不过这有一个问题。我们返回了一个独立的usize，不过它只在&String的上下文中是一个有意义的数字。换句话说，因为它是一个与String相分离的值，无法保证将来它依然有效。
例如：
fn main() {
    let mut s = String::from("hello world!");
    let word = first_word(&s);//word 的值为5
    s.clear();
    //word在此处的值仍然是5，
    //但是没有更多的字符串让我们可以有效地应用数值5.word的值现在完全无效！
}
这个程序编译时没有任何问题，而且在调用s.clear()之后使用word也不会出错。因为word与s状态完全没有联系，所以word仍然包含值5。可以尝试用值5来提取s的第一个单词，不过这是有bug的，因为在我们将5保存到word之后s的内容已经改变。
我们不得不时刻担心word的索引与s中的数据不再同步，这很啰嗦且易出错！如果编写这么一个second_word函数的话，管理索引这件事将更加容易出问题，它的签名看起来像这样：
fn second_word(s: &String) -> (usize, usize) {
    //...
}
现在我们要跟踪一个开始索引和一个结尾索引，同时有了更多从数据的某个特定状态计算而来的值，但都完全没有与这个状态相关联。现在有3个飘忽不定的不相关变量需要保持同步。
幸运的是，rust为这个问题提供了一个解决方法：字符串Slice。


字符串slice
是String中一部分值的引用，它看起来像这样：
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
不同于整个String的引用，hello是一个部分String的引用，由一个额外的[0..5]部分指定。可以使用一个由中括号中的[starting_index, ending_index]指定range创建一个slice，其中starting_index是slice的第一个位置，ending_index则是slice最后一个位置的后一个值。
在其内部，slice的数据结构存储了slice的开始位置和长度，长度对应于ending_index减去starting_index的值。所以对于let world=&s[6..11];的情况，world将是一个包含指向s索引6的指针和长度值5的slice。


对于rust的..range语法，如果想要从索引0开始，可以不写两个点号之前的值。换句话说下面两个语句是相同的：
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

以此类推，如果slice包含String的最后一个字节，也可以舍弃尾部的数字，这意味着如下也是相同的：
let s = String::from("hello");
let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

也可以同时舍弃这两个值来获取整个字符串的slice。所以如下亦是相同的：
let s = String::from("hello");
let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
注意：字符串slice range的索引必须位于有效的UTF-8字符边界内，如果尝试从一个多字节字符的中间位置创建字符串slice，则程序将会因错误而退出。出于介绍字符串slice的目的，本部分假设只使用ASCII字符集；

在记住所有这些知识后，让我们重写first_word来返回一个slice。字符串slice的类型生命写作&str。
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
现在当调用first_word时，会返回与底层数组关联的单个值。这个值由一个slice开始位置的引用和slice中元素的数量组成。
现在我们有了一个不易混淆且直观的API了，因为编译器会确保指向String的引用持续有效。还记得之前程序中，当我们获取第一个单词结尾的索引后，接着清除字符串导致索引无效的bug吗？那些代码在逻辑上是不正确的，但却没有显示任何直接的错误。问题会在之后尝试对空字符串使用第一个单词的引用时出现。slice就不可能出现这种bug并让我们更早的知道出问题了。使用slice版本的first_word会抛出一个编译时错误：
fn main() {
    let mut s = String::from("hello world");
    let world = first_word(&s);
    s.clear();//错误！
    println!("the first word is: {}", word);
}
会发生编译错误，cannoy borrow 's' as mutable because it is also borrowed as immutable
回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。因为clear需要清空String，它尝试获取一个可变引用。在调用clear之后的println!使用了word中的引用，所以这个不可变引用在此时必须仍然有效。rust不允许clear中的可变引用和word中的不可变引用同时存在，因此编译失败。rust不仅使得我们的API简单易用，也在编译时就消除了一整类的错误！


字符串字面值就是slice
字符串字面值被储存在二进制文件中？现在知道slice了，我们就可以正确的理解字符串字面量了：
let s = "hello, world!";
这里s的类型是&str: 它是一个指向二进制程序特定位置的slice。这也就是为什么字符串字面量是不可变的；&str是一个不可变的引用。

字符串slice作为参数
在知道了能够获取字面值和String的slice后，我们对first_word做了改进，这是它的签名：
fm first_word(s: &String) -> &str {
    //...
}

而更有经验的rustacean会编写出以下的签名，因为它使得可以对String值和&str值使用相同的函数：
fn first_word(s: &str) -> &str {
    //...
}

如果有一个字符串slice，可以直接传递它。如果有一个String，则可以传递整个String的slice或对String的引用。这种灵活性利用了deref coercions的优势，这个特性在“函数和方法的隐式Deref强制转换”章节中介绍。定义一个获取字符串slice而不是String引用的函数使得我们的API更加通用并且不会丢失任何功能：
定义一个获取字符串slice而不是String引用的函数使得我们的API更加通用并且不会丢失任何功能：
fn main() {
    let my_string = String::from("hello world");
    //first_word 适用于 String(的slice)，整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // first_word 也适用于String的引用
    // 这等价于整个String的slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    //first_word 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    //因为字符串字面量已经是字符串slice了，
    //这也是适合的，无需slice语法！
    let word = first_word(my_string_literal);
}



其他类型的slice
还有更加通用的slice类型。
let a = [1,2,3,4,5];
let slice = &a[1..3];
assert_eq!(slice, &[2,3]);

这个slice的类型是&[i32]。它跟字符串slice的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。你可以对其他所有集合使用这类sluce。讲到vector时会详细讨论这些集合。



总结

所有权、借用和slice这些概念让rust程序在编译时确保内存安全。rust语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。
所有权系统影响了rust中很多其他部分的工作方式，所以我们还会继续讲到这些概念，这将观察本书的余下内容。让我们开始第5章，来看看如何将多份数据组成进一个struct中。

*/
fn main() {
    println!("Hello, world!");
    let s = String::from("hello");//s进入作用域
    takes_ownership(s);//s的值移动到函数里
    // println!("{}", s);
    let x = 5;//x进入作用域
    makes_copy(x);//x应该移动函数里，但i32是Copy的，所以后面可继续使用x

    let mut s = String::from("hello");

    let r1 = &s;//没问题
    let r2 = &s;//没问题
    println!("{} and {}", r1, r2);
    //此位置之后r1和r2不再使用
    println!("{}", r2);

    let r3 = &mut s;//没问题
    println!("{}", r3);

    let mut s = String::from("hello world");
    let world = first_word_2(&s);
    s.clear();//错误！
    println!("the first word is: {}", word);
}

fn takes_ownership(some_string: String) {//some_string进入作用域
    println!("{}", some_string)
}//这里,some_string移出作用域并调用drop方法。
//占用内存被释放

fn makes_copy(some_integer: i32) {//some_integer 进入作用域
    println!("{}",some_integer)
}//这里,some_integer 移出作用域，没有特殊之处

//返回单词结尾的字节索引
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}