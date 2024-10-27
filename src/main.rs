mod parser;

fn main() {
    println!("Hello, world!");

    let expr_str = "!(A & B) | (C & D)";
    let expr_str = "a&b&c&d";
    let result = parser::parse_expr(expr_str);

    match result {
        Ok((_, expr)) => println!("{:#?}", expr),
        Err(e) => println!("Error: {:?}", e),
    }
}


#[test]
mod tests {
    use crate::parser::parse_expr;

    fn simple() {
        let result = parse_expr("a&b&c&d");

        match result {
            Ok((_, expr)) => println!("{:#?}", expr),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}