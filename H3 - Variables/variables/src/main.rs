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

    
    let x = fahrenheit_to_celcius(40.0);
    println!("40 degree fahrenheit is {} degrees celsius",x );

    let _x = fibonacci_number(20);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
    }

fn fahrenheit_to_celcius( f: f64) -> f64 {
    (( f ) - 32.0) * (5.0/9.0)
}

fn fibonacci_number(n:i64) -> i64 {
    let mut a:i64 = 0;
    let mut b:i64= 1;

    for _num in 1..n {
        let mut p = b;
        b = a + p;
        a = p;        
    }

    println!("The {}th Fibonachi number is: {}",n ,b );
    b
}

