fn main() {
    //21℉=-6.11℃
    let t1 = c2f(-6.11f32);
    assert_eq!(t1, 21.001999f32);
    let t2 = f2c(21.01999f32);
    assert_eq!(t2, -6.100005f32);
    //fibonacci
    let t3 = fibonacci(3);
    assert_eq!(t3, 3);
    let t4 = fibonacci(4);
    assert_eq!(t4, 5);
    let n = 9;
    let res = fibonacci_arr(n);
    for i in 0..9 {
        println!("{}", res[i]);
    }
    /*
    1
    1
    2
    3
    5
    8
    13
    21
    34
    */
    ch();
}

//圣诞颂歌 The Twelve days of Christmas
fn ch() {
//（在圣诞节的第一天，我的真爱送我：一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第二天，我的真爱送我：两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第三天，我的真爱送我：三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第四天，我的真爱送我：四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第五天，我的真爱送我：五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第六天，我的真爱送我：六只生蛋的鹅、五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第七天，我的真爱送我：七只游水的天鹅、六只生蛋的鹅、五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第八天，我的真爱送我：八位挤奶的佣妇、七只游水的天鹅、六只生蛋的鹅、五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第九天，我的真爱送我：九位跳舞的女士、八位挤奶的佣妇、七只游水的天鹅、六只生蛋的鹅、五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第十天，我的真爱送我：十个跳跃的男人、九位跳舞的女士、八位挤奶的佣妇、七只游水的天鹅、六只生蛋的鹅、五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第十一天，我的真爱送我：十一个吹风笛的风笛手、十个跳跃的男人、九位跳舞的女士、八位挤奶的佣妇、七只游水的天鹅、六只生蛋的鹅、五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
//（在圣诞节的第十二天，我的真爱送我：十二个打鼓的鼓手、十一个吹风笛的风笛手、十个跳跃的男人、九位跳舞的女士、八位挤奶的佣妇、七只游水的天鹅、六只生蛋的鹅、五只金戒指、四只鸣唱的鸟儿、三只法国母鸡、两只鸠、及一只站在梨树上的鹧鸪鸟。）
    let items: [&str; 12] = ["Twelve drummers drumming", "Eleven pipers piping", "Ten lords a-leaping", "Nine ladies dancing", "Eight maids a-milking", "Seven swans a-swimming", "Six geese a-laying", "Five golden rings", "Four calling birds", "Three French hens", "Two turtle doves", "a partridge"];
    println!("{:?}", items);
    for i in 0..12 {
        print!("On the first day of Christmas, my true love sent to me:  ");
        for j in 0..=i {
            if j == 0 {
                print!("And {}", items[j]);
            } else {
                print!("{}, ", items[j]);
            }
        }
        print!(" in a pear tree.\n");
    }
}

//n阶斐波那契数列
fn fibonacci_arr(n: i32) -> [i32; 10] {
    let mut arr = [1; 10];
    for i in 0i32..n {
        let j = i as usize;
        arr[j] = fibonacci(i);
    }
    arr
}
//n阶斐波那契数
fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}
//摄氏度转华氏度
fn c2f(c: f32) -> f32 {
    c * 1.8 + 32f32
}
//华氏度转摄氏度
fn f2c(f: f32) -> f32 {
    ( f - 32f32 ) / 1.8
}