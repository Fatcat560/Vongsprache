
mod lexer;
use lexer::Lexer;
use lexer::Token;


fn main(){
    let mut lex = Lexer::new("i bims Arc vong \"Programmierer\" her".to_string());
    let mut x = lex.get_next_token();
    while x != Token::EOF {
        println!("{:?}", x);
        x = lex.get_next_token();
    }
}