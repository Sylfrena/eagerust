
use crate::token;

struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: &'static str,
}


impl Lexer{
    fn new(input: String) ->  Lexer {
        let mut lo =  Lexer{input: input,
                                position: 0,
                                read_position: 0,
                                ch: "\0",
                  };
        lo.read_char();
        return lo
    }

    fn read_char(&mut self)  {
        if self.read_position >=  self.input.len() as i32{
            self.ch = "0" ;
        } else {
            //.chars().nth(self.read_position as usize).unwrap().to_string();
            self.ch = &self.input[self.read_position-1..self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;

    }

    fn next_token(&mut self) -> token::Token {

        let tok = match self.ch{
                               "=" => new_token(token::ASSIGN, self.ch),
                               "+" => new_token(token::PLUS, self.ch),
                               "-" => new_token(token::MINUS, self.ch),
                               "/" => new_token(token::DIVIDE, self.ch),
                               "*" => new_token(token::MULTIPLY, self.ch),
                               "%" => new_token(token::MODULO, self.ch),
                               ">" => new_token(token::GREATER_THAN, self.ch),
                               "<" => new_token(token::LESS_THAN, self.ch),
                               "!" => new_token(token::NOT, self.ch),
                               "," => new_token(token::COMMA, self.ch),
                               ";" => new_token(token::SEMICOLON, self.ch),
                               "?" => new_token(token::QUESTION, self.ch),
                               "(" => new_token(token::LPAREN, self.ch),
                               ")" => new_token(token::RPAREN, self.ch),
                               "[" => new_token(token::LSQUARE, self.ch),
                               "]" => new_token(token::RSQUARE, self.ch),
                               "{" => new_token(token::LCURLY, self.ch),
                               "}" => new_token(token::RCURLY, self.ch),
                               _ => new_token(token::EOF, ""),
        };

        self.read_char(); //references are not mutable, take care of this, will show problems in read_char()
        tok
    }
}

fn new_token(token_symbol: token::TokenType, byte: &'static str) -> token::Token {
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

        struct Expected<'a> {
            expected_type: token::TokenType,
            expected_literal: &'a str,
        }
        
        let input = String::from("five =5;") ;
        let mut lo = Lexer::new(input) ;//check ownership
        
        let mut test = vec![
           // Expected{expected_type : token::LET, expected_literal: "let"},
            //Expected{expected_type : token::ASSIGN, expected_literal: '='},
            //Expected{expected_type : token::ASSIGN, expected_literal: '='},
            //Expected{expected_type : token::ASSIGN, expected_literal: '='},
            Expected{expected_type : token::IDENT, expected_literal: "five"},

            Expected{expected_type : token::ASSIGN, expected_literal: "="},
            Expected{expected_type : token::INT, expected_literal: "5"},
           // Expected{expected_type : token::PLUS, expected_literal: '+'},
           // Expected{expected_type : token::MINUS, expected_literal: '-'},
           // Expected{expected_type : token::DIVIDE, expected_literal: '/'},
           // Expected{expected_type : token::MULTIPLY, expected_literal: '*'},
           // Expected{expected_type : token::MODULO, expected_literal: '%'},
           // Expected{expected_type : token::GREATER_THAN, expected_literal: '>'},
           // Expected{expected_type : token::LESS_THAN, expected_literal: '<'},
           // Expected{expected_type : token::NOT, expected_literal: '!'},
           // Expected{expected_type : token::COMMA, expected_literal: ','},
            Expected{expected_type : token::SEMICOLON, expected_literal: ";"},
            //Expected{expected_type : token::QUESTION, expected_literal: '?'},
            //Expected{expected_type : token::LPAREN, expected_literal: '('},
            //Expected{expected_type : token::RPAREN, expected_literal: ')'},
            //Expected{expected_type : token::LSQUARE, expected_literal: '['},
            //Expected{expected_type : token::RSQUARE, expected_literal: ']'},
            //Expected{expected_type : token::LCURLY, expected_literal: '{'},
            //Expected{expected_type : token::RCURLY, expected_literal: '}'}
        ];


        for el in test.iter() {
            let tok = lo.next_token();
            assert_eq!(el.expected_type,  tok.token_type);
            assert_eq!(el.expected_literal, tok.literal);
        }

   }

}



