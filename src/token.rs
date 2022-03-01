//! Polar TOKENS.
//! This file implements tokens.

use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Hash)]
pub enum TokenKind {
    LiteralStr, // Literal Strings.
    LiteralInt, // Literal Integers.

    Identifier, // Identifier tokens (Variables).
    Function,   // Function defition. `func`
    While,      // While statement. `while`
    For,        // For statement `for`

    OpAssign, // Assign. `=`
    OpAdd,    // Addition. `+`
    OpSub,    // Subtraction. `-`
    OpMul,    // Multiplication. `*`
    OpDiv,    // Division. `/`
    OpMod,    // Modulo. `%`

    LParen, // Left parenthesis. `(`
    RParen, // Right parenthesis. `)`
    LBrace, // Left brace. `{`
    RBrace, // Right brace. `}`
    LBrack, // Left bracket. `(`
    RBrack, // Right bracket. `)`

    Semi,    // Semi colon.
    Invalid, // Invalid tokens.
    EOS,     // END OF SOURCE.
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::LiteralStr => f.write_str("STR"),
            TokenKind::LiteralInt => f.write_str("INT"),
            TokenKind::Identifier => f.write_str("ID"),

            TokenKind::Function => f.write_str("FUNC"),
            TokenKind::While => f.write_str("WHILE"),
            TokenKind::For => f.write_str("FOR"),

            TokenKind::OpAssign => f.write_str("="),
            TokenKind::OpAdd => f.write_str("+"),
            TokenKind::OpSub => f.write_str("-"),
            TokenKind::OpMul => f.write_str("*"),
            TokenKind::OpDiv => f.write_str("/"),
            TokenKind::OpMod => f.write_str("%"),

            TokenKind::LParen => f.write_str("("),
            TokenKind::RParen => f.write_str(")"),
            TokenKind::LBrace => f.write_str("{"),
            TokenKind::RBrace => f.write_str("}"),
            TokenKind::LBrack => f.write_str("["),
            TokenKind::RBrack => f.write_str("]"),

            TokenKind::Semi => f.write_str(";"),
            TokenKind::Invalid => f.write_str("INVALID"),
            TokenKind::EOS => f.write_str("END-OF-SOURCE"),
        }
    }
}

pub struct Token {
    value: String,
    kind: TokenKind,
}

#[allow(dead_code)]
impl Token {
    pub fn new(value: String, kind: TokenKind) -> Token {
        return Token {
            value: value,
            kind: kind,
        };
    }

    pub fn to_string(&self) -> String {
        return self.kind.to_string();
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({}, {})", self.value, self.kind)
    }
}
