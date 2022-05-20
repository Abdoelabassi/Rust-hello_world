
use::std::env::{args, Args};

fn main() {

    let mut args: Args = args();

    let first = args.nth(1).unwrap();

    let operator = args.nth(0).unwrap().chars().next().unwrap();

    let second = args.nth(0).unwrap();


    let first_number = first.parse::<f32>().unwrap();

    let second_number = second.parse::<f32>().unwrap();

    let result = operate(operator, first_number, second_number);

    println!("{:?}", output(first_number, operator, second_number, result));
}

fn operate(op:char, x:f32, y:f32)-> f32{

    match op {
        '+' => x + y,
        '-' => x - y,
        '*' | 'x' | 'X' => x*y,
        '/' => x/y,

        _ => panic!("invalid operator.")
    }
}


fn output(x:f32, op:char, y:f32, res:f32)->String {

    format!("{} {} {} = {} ", x, op, y, res)
}
