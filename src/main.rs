use std::i64;
fn main() {
    println!("Insert type [2,8,10,16]");
    let mytype = stou(read_string());

    println!("Insert first number");
    let mystring1 = read_string();
    let num1 = converter(mystring1, mytype);

    println!("Insert second number");
    let mystring2 = read_string();
    let num2 = converter(mystring2, mytype);
    
    println!("Insert operation [+,-,*]");
    let operation = read_string();
    let mut result = 0;
    if operation == "+"
    {
        result = num1+num2;
    }
    if operation == "-"
    {
        result = num1-num2;
    }
    if operation == "*"
    {
        result = num1*num2;
    }

    if mytype == 2
    {
        println!("{:b}", result);
    }
    if mytype == 8
    {
        println!("{:o}", result);
    }
    if mytype == 16
    {
        println!("{:x}", result);
    }
    if mytype == 10
    {
        println!("{}", result);
    }
}

fn read_string() -> String
{
    let mut value = String::new();
    std::io::stdin().read_line(&mut value).expect("oops!");
    value = value.trim().to_string();
    value
}

fn stoi(num: String) -> i64
{
    let out:i64 = num.trim().parse::<i64>().expect("oopppppssss!");
    println!("{}",out);
    out
}

fn stou(num: String) -> u32
{
    let out:u32 = num.trim().parse::<u32>().expect("oopppppssss!");
    out
}

fn converter(num: String, ntype: u32) -> i64
{   
    let newnum = i64::from_str_radix(&num, ntype).unwrap();
    newnum
}
