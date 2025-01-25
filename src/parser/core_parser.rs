use std::fmt::Debug;

// use crate::parser::token::*;
use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;

pub enum Prio {
    Left,
    Right,
    Prefix,
}

pub struct Ope<'a> {
    pub opestr: &'a str,
    pub priority_direction: Prio,
    pub priority: i32,
}

/// macro for Parser trait
macro_rules! def_ope {
    ($name:ident,$string:expr,$prio_direction:path,$prio:expr) => {
        const $name: &'a Ope<'a> = &Ope {
            opestr: $string,
            priority_direction: $prio_direction,
            priority: $prio,
        };
    };
}

pub enum OpeTable {
    ARROW, // 矢印
    OR,
    AND,
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    DOT,
    ASSIGNMENT,
    ADDEQ,
    SUBEQ,
    MULEQ,
    DIVEQ,
    MODEQ,
    POW,
    NOT,
}

impl OpeTable {
    pub fn set(s: &str) -> Result<Self, &str> {
        match s {
            "->" => Ok(Self::ARROW),
            "||" => Ok(Self::OR),
            "&&" => Ok(Self::AND),
            "==" => Ok(Self::EQ),
            "!=" => Ok(Self::NE),
            "<" => Ok(Self::LT),
            "<=" => Ok(Self::LE),
            ">" => Ok(Self::GT),
            ">=" => Ok(Self::GE),
            "+" => Ok(Self::ADD),
            "-" => Ok(Self::SUB),
            "*" => Ok(Self::MUL),
            "/" => Ok(Self::DIV),
            "%" => Ok(Self::MOD),
            "@" => Ok(Self::DOT),
            "=" => Ok(Self::ASSIGNMENT),
            "+=" => Ok(Self::ADDEQ),
            "-=" => Ok(Self::SUBEQ),
            "*=" => Ok(Self::MULEQ),
            "/=" => Ok(Self::DIVEQ),
            "%=" => Ok(Self::MODEQ),
            "**" => Ok(Self::POW),
            "!" => Ok(Self::NOT),
            _ => Err("Invalid Operation String"),
        }
    }
}

/// # Parser trait
/// パーサのコア実装
pub trait Parser<'a> {
    // operators
    // - left priority
    //   - priority -3
    def_ope!(ARROW, "->", Prio::Right, -4);
    def_ope!(OR, "||", Prio::Left, -3);
    //   - priority -2
    def_ope!(AND, "&&", Prio::Left, -2);
    //   - priority 0
    def_ope!(EQ, "==", Prio::Left, 0);
    def_ope!(NE, "!=", Prio::Left, 0);
    def_ope!(LT, "<", Prio::Left, 0);
    def_ope!(LE, "<=", Prio::Left, 0);
    def_ope!(GT, ">", Prio::Left, 0);
    def_ope!(GE, ">=", Prio::Left, 0);
    //   - priority 1
    def_ope!(ADD, "+", Prio::Left, 1);
    def_ope!(SUB, "-", Prio::Left, 1);
    def_ope!(MUL, "*", Prio::Left, 2);
    def_ope!(DIV, "/", Prio::Left, 2);
    def_ope!(MOD, "%", Prio::Left, 2);
    def_ope!(DOT, "@", Prio::Left, 2);

    // - right priority
    //   - priority -4
    def_ope!(ASSIGNMENT, "=", Prio::Right, -5);
    def_ope!(ADDEQ, "+=", Prio::Right, -5);
    def_ope!(SUBEQ, "-=", Prio::Right, -5);
    def_ope!(MULEQ, "*=", Prio::Right, -5);
    def_ope!(DIVEQ, "/=", Prio::Right, -5);
    def_ope!(MODEQ, "%=", Prio::Right, -5);
    //   - priority -3
    def_ope!(POW, "**", Prio::Right, 3);

    // - prefix priority
    //   - priority -1
    def_ope!(NOT, "!", Prio::Prefix, -1);

    /// 演算子を文字列として長いものからの順番で並べたもの
    const LENGTH_ORDER_OPE_LIST: [&'a Ope<'a>; 23] = [
        // length 2
        Self::ARROW,
        Self::OR,    // ||
        Self::AND,   // &&
        Self::EQ,    // ==
        Self::NE,    // !=
        Self::LE,    // <=
        Self::GE,    // >=
        Self::ADDEQ, // +=
        Self::SUBEQ, // -=
        Self::MULEQ, // *=
        Self::DIVEQ, // /=
        Self::MODEQ, // %=
        Self::POW,   // **
        // length 1
        Self::LT,         // <
        Self::GT,         // >
        Self::ADD,        // +
        Self::SUB,        // -
        Self::MUL,        // *
        Self::DIV,        // /
        Self::MOD,        // %
        Self::DOT,        // @
        Self::ASSIGNMENT, // =
        Self::NOT,        // !
    ];

    // comment
    const COMMENT_OPEN: &'a str = "/*";
    const COMMENT_CLOSE: &'a str = "*/";
    const COMMENT_START: &'a str = "//"; // end newline or

    const SEMICOLON: char = ';';
    const COMMA: char = ',';
    const SPLIT_CHAR: [char; 3] = [' ', '\t', '\n'];
    const EXCLUDE_WORDS: [char; 3] = [Self::SEMICOLON, ':', Self::COMMA];

    const SYNTAX_IF: &'a str = "if";
    const SYNTAX_ELIF: &'a str = "elif";
    const SYNTAX_ELSE: &'a str = "else";
    const SYNTAX_LOOP: &'a str = "loop";
    const SYNTAX_FOR: &'a str = "for";
    const SYNTAX_WHILE: &'a str = "while";
    const SYNTAX_MATCH: &'a str = "match";

    const SYNTAX_WORDS: [&'a str; 7] = [
        Self::SYNTAX_IF,    // if   (){}
        Self::SYNTAX_ELIF,  // elif (){}
        Self::SYNTAX_ELSE,  // else {}
        Self::SYNTAX_LOOP,  // loop {}
        Self::SYNTAX_FOR,   // for  (){}
        Self::SYNTAX_WHILE, // while(){}
        Self::SYNTAX_MATCH, // match(){}
    ];
    const SYNTAX_WORDS_HEADS: [&'a str; 4] = [
        Self::SYNTAX_IF,    // if   (){} ...
        Self::SYNTAX_LOOP,  // loop (){} ...
        Self::SYNTAX_FOR,   // for  (){} ...
        Self::SYNTAX_WHILE, // while(){} ...
    ];
    const ESCAPECHAR: char = '\\';
    const FUNCTION: &'a str = "fn";
    const PUB_FUNCTION: &'a str = "pub_fn";
    const STRUCTURE: &'a str = "struct";
    const ENUMERATION: &'a str = "enum";
    const DOUBLE_QUOTATION: char = '"';
    const SINGLE_QUOTATION: char = '\'';

    const CONTROL_RETURN: &'a str = "return";
    const CONTROL_BREAK: &'a str = "break";
    const CONTROL_CONTINUE: &'a str = "continue";
    const CONTROL_ASSERT: &'a str = "assert";
    const CONTROL_LET: &'a str = "let";
    const CONTROL_LETMUT: &'a str = "let_mut";

    const CONTROL_STATEMENT: [&'a str; 6] = [
        Self::CONTROL_RETURN,   // return
        Self::CONTROL_BREAK,    // break
        Self::CONTROL_CONTINUE, // continue
        Self::CONTROL_ASSERT,   // assert
        Self::CONTROL_LET,      // let
        Self::CONTROL_LETMUT,   // let mut
    ];

    const KEYWORDS: [&'a str; 16] = [
        // Syntax
        Self::SYNTAX_IF,    // if
        Self::SYNTAX_ELIF,  // elif
        Self::SYNTAX_ELSE,  // else
        Self::SYNTAX_LOOP,  // loop
        Self::SYNTAX_FOR,   // match
        Self::SYNTAX_WHILE, // while
        Self::SYNTAX_MATCH, // match
        // keyword
        Self::FUNCTION,    // fn
        Self::STRUCTURE,   // struct
        Self::ENUMERATION, // enum
        // control
        Self::CONTROL_RETURN,   // return
        Self::CONTROL_BREAK,    // break
        Self::CONTROL_CONTINUE, // control
        Self::CONTROL_ASSERT,   // assert
        Self::CONTROL_LET,      // let
        Self::CONTROL_LETMUT,   // let_mut
    ];

    const BLOCK_BRACE_OPEN: char = '{';
    const BLOCK_BRACE_CLOSE: char = '}';
    const BLOCK_PAREN_OPEN: char = '(';
    const BLOCK_PAREN_CLOSE: char = ')';
    const BLOCK_LIST_OPEN: char = '[';
    const BLOCK_LIST_CLOSE: char = ']';
    // type
    const BLOCK_TYPE_OPEN: char = '<';
    const BLOCK_TYPE_CLOSE: char = '>';

    fn new(code: String, depth: isize, loopdepth: isize) -> Self;
    fn resolve(&mut self) -> Result<(), ParserError>;
    fn code2_vec_pre_proc_func<T>(code: &str) -> Vec<T>
    where
        T: Token + Clone + Debug,
    {
        return code
            .chars()
            .map(|c| Token::set_char_as_unknown(c))
            .collect();
    }

    fn find_ope_priority(ope: &'a str) -> Result<&Ope, &'a str> {
        for i in Self::LENGTH_ORDER_OPE_LIST {
            if i.opestr == ope {
                return Ok(i);
            }
        }
        Err("ope not exist")
    }
}
