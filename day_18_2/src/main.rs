use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("File I/O error");
    let line_seperated_chars: Vec<Vec<char>> = input_file
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect();

    let mut expressions: Vec<Vec<Token>> = vec![];
    for line in line_seperated_chars.iter() {
        let mut num_string = String::new();
        let mut expression: Vec<Token> = vec![];
        for c in line.iter() {
            if c.is_numeric() {
                num_string.push(*c);
                continue;
            }

            if !num_string.is_empty() {
                expression.push(Token::Num(num_string.parse::<u64>().unwrap()));
                num_string.clear();
            }

            expression.push(match c {
                '+' => Token::Sum,
                '*' => Token::Product,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                _ => panic!("Token={}", c),
            })
        }

        if !num_string.is_empty() {
            expression.push(Token::Num(num_string.parse::<u64>().unwrap()));
        }

        expressions.push(expression);
    }

    println!(
        "{}",
        expressions
            .into_iter()
            .map(|exp| evalute(&to_postfix(&exp)))
            .sum::<u64>()
    )
}

#[derive(PartialEq, Clone, Copy)]
enum Token {
    Sum,
    Product,
    LeftParen,
    RightParen,
    Num(u64),
}

fn to_postfix(infix: &Vec<Token>) -> Vec<Token> {
    let mut postfix: Vec<Token> = vec![];
    let mut operators: Vec<Token> = vec![];

    for token in infix.iter() {
        match *token {
            Token::Sum => {
                let mut op;
                loop {
                    if operators.is_empty() {
                        break;
                    }
                    op = operators.pop().unwrap();
                    if op == Token::LeftParen || op == Token::Product {
                        operators.push(op);
                        break;
                    }
                    postfix.push(op);
                }
                operators.push(*token);
            }
            Token::Product => {
                let mut op;
                loop {
                    if operators.is_empty() {
                        break;
                    }
                    op = operators.pop().unwrap();
                    if op == Token::LeftParen {
                        operators.push(Token::LeftParen);
                        break;
                    }
                    postfix.push(op);
                }
                operators.push(*token);
            }
            Token::LeftParen => operators.push(*token),
            Token::RightParen => {
                let mut op = operators.pop().unwrap();
                while op != Token::LeftParen {
                    postfix.push(op);
                    op = operators.pop().unwrap();
                }
            }
            Token::Num(_) => postfix.push(*token),
        }
    }

    // Remaining operators
    while !operators.is_empty() {
        postfix.push(operators.pop().unwrap());
    }

    // Debug
    for token in postfix.iter() {
        match *token {
            Token::Num(x) => print!("{}", x),
            Token::Sum => print!("+"),
            Token::Product => print!("*"),
            Token::LeftParen => print!("("),
            Token::RightParen => print!(")"),
        }
        print!(" ");
    }

    postfix
}

fn evalute(expression: &Vec<Token>) -> u64 {
    let mut operands: Vec<u64> = vec![];

    for token in expression.iter() {
        match *token {
            Token::Num(x) => operands.push(x),
            Token::Sum => sum(&mut operands),
            Token::Product => product(&mut operands),
            _ => (),
        }
    }
    let ans = operands.pop().unwrap();
    println!(" => {}", ans);
    ans
}

fn sum(operands: &mut Vec<u64>) {
    let a = operands.pop().unwrap();
    let b = operands.pop().unwrap();
    // println!("{} {} +", a, b);
    operands.push(a + b);
}

fn product(operands: &mut Vec<u64>) {
    let a = operands.pop().unwrap();
    let b = operands.pop().unwrap();
    // println!("{} {} *", a, b);
    operands.push(a * b);
}
