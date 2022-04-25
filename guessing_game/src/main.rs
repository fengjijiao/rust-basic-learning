use std::io;
use rand::Rng;
use std::cmp::Ordering;
// the program wil generate a random integer between 1 and 100, It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. if the guess is corrent, the game will print a congratulatory message and exit.
fn main() {
    println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..101);
    let secret_number = rand::thread_rng().gen_range(1..=100);//type: i32

    // println!("the secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to reead line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                //当输入的非数字时，可以继续guess
                println!("Please type a number!");
                continue;
            }
        };
        //字符串的trim方法会去除字符串开头和结尾的空白字符，用户必须输入enter键才能让read_line返回并输入他们的guess，这将会在字符串中增加一个换行符。（在win中，按下enter键将会得到一个回车符和一个换行符，\r\n），trim方法会消除\n或者\r\n。
        //字符串的parse方法只有在字符逻辑上可以转换为数字的时候才能工作所以非常容易出错，因此parse方法会返回一个Result类型。

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //当猜对的时候退出循环
                break;
            },
        }
    }

    // let mut gg2 = String::new();
    // let result = io::stdin().read_line(&mut gg2);

    // println!("gg2: {}, result: {:?}", gg2, result);

}
