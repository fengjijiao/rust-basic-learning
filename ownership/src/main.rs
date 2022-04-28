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


*/
fn main() {
    println!("Hello, world!");
}