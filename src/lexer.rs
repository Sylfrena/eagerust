
use crate::token;

struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char,
}


impl Lexer{
    fn new(input: String) ->  Lexer {
        Lexer{input: input,
                    position: 0,
                    read_position: 0,
                    ch: '\0',
        }
    } 

    fn next_token(&self) -> token::Token {

        let tok = match self.ch {
            '=' =>  new_token(token::ASSIGN, self.ch),
            '-' =>  new_token(token::MINUS, self.ch),
            ',' =>  new_token(token::COMMA, self.ch),
            '+' =>  new_token(token::PLUS, self.ch),
            '/' =>  new_token(token::DIVIDE, self.ch),
            '*' =>  new_token(token::MULTIPLY, self.ch),
            ')' =>  new_token(token::RPAREN, self.ch),
            _ =>  new_token(token::EOF, '\0' ), 
        };


        //self.read_char(); //references are not mutable, take care of this, will show problems in read_char()
        tok
    }
}

fn new_token(token_symbol: token::TokenType, byte: char) -> token::Token {
    token::Token{token_type:  token_symbol, 
                             literal: byte }
}


//UNIT TESTS

//#[derive(Debug)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lexer() {

        struct Expected {
            expected_type: token::TokenType,
            expected_literal: char,
        }
        
        let input = String::from("=-,+/*)") ;
        let lo = Lexer::new(input) ;//check ownership
        
        let mut test = vec![
            Expected{expected_type : token::ASSIGN, expected_literal: '='},
            Expected{expected_type : token::MINUS, expected_literal: '-'},
            Expected{expected_type : token::ASSIGN, expected_literal: ','},
            Expected{expected_type : token::PLUS, expected_literal: '+'},
            Expected{expected_type : token::DIVIDE, expected_literal: '/'},
            Expected{expected_type : token::MULTIPLY, expected_literal: '*'},
            Expected{expected_type : token::RPAREN, expected_literal: ')'},
        ];


        for el in test.iter() {

            let tok = lo.next_token();
            assert_eq!(el.expected_type,  tok.token_type);
            assert_eq!(el.expected_literal, tok.literal);

        }

   }

}



