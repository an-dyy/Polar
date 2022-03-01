mod lexer;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new("Foo bar");

    for tok in lexer.scan() {
        println!("{}", tok);
    }
}
