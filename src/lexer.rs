use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Token{
    Integer(i32),
    Real(f32),
    String(String),
    Plus,
    Minus,
    Mul,
    IntegerDiv,
    RealDiv,
    LParen,
    RParen,
    Id(String),
    Comma,
    EOF,
    //RESERVED KEYWORD TOKENS FROM HERE ON
    IToken,
    BimsToken,
    VongToken,
    HerToken,
    MitToken,
    BisRangeToken,
    FunktionigkeitToken,
    AmElseToken,
    WahrigkeitBoolToken,
    SonstigkeitElseToken,
    SolangeWhileToken,
    BiddeFuncCallToken,

}

pub struct Lexer{
    text: String,
    position: usize,
    current_char: Option<char>,
    keywords: HashMap<String, Token>,
}

//FOR NEW KEYWORDS, PLEASE MODIFY THIS current_keywords ARRAY (and add a Token to the enum)
fn fill_map() -> HashMap<String, Token> {
    let current_keywords: [(String, Token); 12] = [
        (String::from("i"), Token::IToken),
        ("bims".to_string(), Token::BimsToken),
        ("vong".to_string(), Token::VongToken),
        ("her".to_string(), Token::HerToken),
        ("mit".to_string(), Token::MitToken),
        ("bis".to_string(), Token::BisRangeToken),
        ("Funktionigkeit".to_string(), Token::FunktionigkeitToken),
        ("am".to_string(), Token::AmElseToken),
        ("Wahrigkeit".to_string(), Token::WahrigkeitBoolToken),
        ("Sonstigkeit".to_string(), Token::SonstigkeitElseToken),
        ("solange".to_string(), Token::SolangeWhileToken),
        ("bidde".to_string(), Token::BiddeFuncCallToken)
    ];
    let mut map:HashMap<String, Token> = HashMap::new();
    for item in current_keywords.iter() {
        let (k,v) = item;
        map.insert(k.to_owned(), v.to_owned());
    }
    map
}

impl Lexer{
    ///Create a new Lexer with given input
    pub fn new(input: String) -> Lexer {
        let map = fill_map();
        let mut lex = Lexer{
            text:input,
            position:0,
            //Give the first character to our lexer or None if the input is empty
            current_char:None,
            keywords: map,
        };
        lex.current_char = lex.text.chars().next();
        lex
    }

    fn error(&mut self){
        panic!("invalid character!");
    }

    fn advance(&mut self){
        self.position += 1;
        self.current_char = self.text.chars().nth(self.position);
    }

    fn peek(&mut self) -> Option<char> {
        let peek_pos = self.position + 1;

        self.text.chars().nth(peek_pos)
    }

    fn ignore_whitespace(&mut self){
        while let Some(character) = self.current_char {
            if character.is_whitespace(){
                self.advance();
            } else {
                break;
            }
        }
    }

    fn ignore_comment(&mut self){
        while let Some(c) =  self.current_char{
            if c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
        self.advance();
    }

    fn number(&mut self) -> Token{
        let token:Token;
        let mut number_string = String::new();
        while let Some(character) = self.current_char {
            if character.is_digit(10) {
                number_string.push(character);
                self.advance();
            } else {
                break;
            }
        }
        if self.current_char == Some('.') {
            number_string.push(self.current_char.unwrap());
            self.advance();
            while let Some(character) = self.current_char {
                if character.is_digit(10) {
                    number_string.push(character);
                    self.advance();
                } else {
                    break;
                }
            }
            token = Token::Real(number_string.parse::<f32>().unwrap());
        } else {
            token = Token::Integer(number_string.parse::<i32>().unwrap());
        }
        token
    }

    fn id(&mut self) -> Token {
        let mut result = String::new();
        while let Some(character) = self.current_char {
            if character.is_alphanumeric() {
                result.push(character);
                self.advance();
            } else {
                break;
            }
        }
        match self.keywords.get(&result) {
            Some(val) => {
                let token = val.to_owned();
                return  token.clone();
            },
            None => {
                return Token::Id(result);
            },
        };
    }

    fn string(&mut self) -> Token {
        let mut result = String::new();
        while let Some(character) = self.current_char {
            if character != '"' {
                result.push(character);
                self.advance();
            } else {
                self.advance();
                break;
            }
        }
        return Token::String(result);
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(character) = self.current_char{
            if character.is_whitespace() {
                self.ignore_whitespace();
                continue;
            }

            if character.is_digit(10){
                return self.number();
            }

            if character.is_alphanumeric() {
                return self.id();
            }

            match character {
                '#' => {
                    self.advance();
                    self.ignore_comment();
                    continue;
                },
                '+' => {
                    self.advance();
                    return Token::Plus;
                },
                '-' => {
                    self.advance();
                    return Token::Minus;
                },
                '*' => {
                    self.advance();
                    return Token::Mul;
                },
                '/' => {
                    self.advance();
                    return Token::RealDiv;
                },
                '$' => { //TODO: Think of a nice way for integer division
                    self.advance();
                    return Token::IntegerDiv;
                },
                '(' => {
                    self.advance();
                    return Token::LParen;
                },
                ')' => {
                    self.advance();
                    return Token::RParen;
                },
                '"' => {
                    self.advance();
                    return self.string();
                },
                ',' => {
                    self.advance();
                    return Token::Comma;
                }
                _ => self.error()
            }
        }
        Token::EOF

    }
}