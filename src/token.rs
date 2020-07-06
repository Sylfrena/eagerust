
pub type TokenType = &'static str;

pub struct Token {
    pub token_type: TokenType,
    pub literal: &'static str,
}

/*We are not specifying &str because compiler will infer the type
 *These constants likely won't get pulled into lifetime messes hence
 *we are okay with using &str instead of String
 */

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

//Identifiers
  
pub const IDENT: TokenType = "IDENTIFIER";
pub const INT: TokenType = "INT";
pub const FLOAT: TokenType = "FLOAT";

// Operators
pub const ASSIGN: TokenType = "= " ;
pub const PLUS: TokenType = "+" ;
pub const MINUS: TokenType = "-" ;
pub const DIVIDE: TokenType = "/" ;
pub const MULTIPLY: TokenType = "*" ;
pub const MODULO: TokenType = "%" ;
//EQUAL_TO: TokenType = '= '
pub const GREATER_THAN: TokenType = ">" ;
pub const LESS_THAN: TokenType = "<" ;
//#UNEQUAL_TO: TokenType = '!
//#AND: TokenType = '&&'
//#OR: TokenType = '||'
pub const NOT: TokenType = "!" ;
  
//# Delimiters
pub const COMMA: TokenType = "," ;
pub const SEMICOLON: TokenType = ";" ;
pub const QUESTION: TokenType = "?" ;
  
pub const LPAREN: TokenType = "(" ;
pub const RPAREN: TokenType = ")" ;
pub const LSQUARE: TokenType = "[" ;
pub const RSQUARE: TokenType = "]" ;
pub const LCURLY: TokenType = "{" ;
pub const RCURLY: TokenType = "}" ;

//Keywords
pub const FUNCTION: TokenType = "def" ;
//SAY: TokenType ='SAY'
