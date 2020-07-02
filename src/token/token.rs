use std::io;

mod token{

type TokenType = String;

struct Token {
    token_type: TokenType,
    literal: String,
}

/*We are not specifying &str because compiler will infer the type
 *These constants likely won't get pulled into lifetime messes hence
 *we are okay with using &str instead of String
 */

const ILLEGAL = "ILLEGAL";
const EOF = "EOF";

//Identifiers
  
const IDENT = "IDENTIFIER";
const INT = "INT";
const FLOAT = "FLOAT";

// Operators
const ASSIGN = "=" ;
const PLUS = "+" ;
const MINUS = "-" ;
const DIVIDE = "/" ;
const MULTIPLY = "*" ;
const MODULO = "%" ;
//EQUAL_TO = '=='
const GREATER_THAN = ">" ;
const LESS_THAN = "<" ;
//#UNEQUAL_TO = '!='
//#AND = '&&'
//#OR = '||'
const NOT = "!" ;
  
//# Delimiters
const COMMA = "," ;
const SEMICOLON = ";" ;
const QUESTION = "?" ;
  
const LPAREN = "(" ;
const RPAREN = ")" ;
const LSQUARE = "[" ;
const RSQUARE = "]" ;
const LCURLY = "{" ;
const RCURLY = "}" ;

//Keywords
const FUNCTION = "def" ;
//SAY = 'SAY'

}