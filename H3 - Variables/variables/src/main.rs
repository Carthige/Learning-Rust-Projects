fn main() {

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}",spaces );

    let _guess: u32 = "43".parse().expect("Not a number!");

    let x = five();
    let _y = plus_one(x);


}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
    }
