use super::error::Error;
use super::token::{Token, TokenType};
use std::iter::Peekable;

pub fn scan_tokens(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line_number = 1;

    while !chars.peek().is_none() {
        match scan_token(&mut chars, &mut line_number) {
	    Ok(ScanResult::Tok(token)) => tokens.push(token),
	    Ok(ScanResult::Comment) => consume_line(&mut chars, &mut line_number),
	    Ok(ScanResult::Whitespace) => (),
	    Err(err) => {
		eprintln!("{err}");
		break;
	    }
	};
    }
    tokens.push(Token::new(TokenType::EOF, 0));

    tokens
}

enum ScanResult {
    Tok(Token),
    Comment,
    Whitespace,
}

fn consume_line<I: Iterator<Item = char>>(chars: &mut Peekable<I>, line_number: &mut usize) {
    loop {
        match chars.next() {
            None => break,
            Some('\n') => {
                *line_number += 1;
                break;
            }
            Some(_) => (),
        }
    }
}

fn scan_token<I: Iterator<Item = char>>(
    chars: &mut Peekable<I>,
    line_number: &mut usize,
) -> Result<ScanResult, Error> {
    let c: char = chars.next().unwrap(); // ok because of peek above

    let token = match c {
	'(' => Token::new(TokenType::LEFT_PAREN, *line_number),
	')' => Token::new(TokenType::RIGHT_PAREN, *line_number),
	'{' => Token::new(TokenType::LEFT_BRACE, *line_number),
	'}' => Token::new(TokenType::RIGHT_BRACE, *line_number),
	',' => Token::new(TokenType::COMMA, *line_number),
	'.' => Token::new(TokenType::DOT, *line_number),
	'-' => Token::new(TokenType::MINUS, *line_number),
	'+' => Token::new(TokenType::PLUS, *line_number),
	';' => Token::new(TokenType::SEMICOLON, *line_number),
	'*' => Token::new(TokenType::STAR, *line_number),
	'!' => {
	    if matches(chars, '=') {
		Token::new(TokenType::BANG_EQUAL, *line_number)
	    } else {
		Token::new(TokenType::BANG, *line_number)
	    }
	}

	'=' => {
	    if matches(chars, '=') {
		Token::new(TokenType::EQUAL_EQUAL, *line_number)
	    } else {
		Token::new(TokenType::EQUAL, *line_number)
	    }
	}
	'<' => {
	    if matches(chars, '=') {
		Token::new(TokenType::LESS_EQUAL, *line_number)
	    } else {
		Token::new(TokenType::LESS, *line_number)
	    }
	}
	'>' => {
	    if matches(chars, '=') {
		Token::new(TokenType::GREATER_EQUAL, *line_number)
	    } else {
		Token::new(TokenType::GREATER, *line_number)
	    }
	}
	'/' => {
	    if matches(chars, '/') {
		return Ok(ScanResult::Comment);
	    } else {
		Token::new(TokenType::SLASH, *line_number)
	    }
	}
	whitespace if whitespace == ' ' || whitespace == '\r' || whitespace == '\t' => {
	    return Ok(ScanResult::Whitespace)
	}
	'\n' => {
	    *line_number += 1;
	    return Ok(ScanResult::Whitespace);
	}
	'"' => Token::new(TokenType::STRING(string(chars, line_number)?), *line_number),
	digit if digit.is_numeric() => {
	    Token::new(TokenType::NUMBER(number(chars, digit)?), *line_number)},
	id if id.is_alphabetic() => identifier(chars, *line_number, id),
	_ => {
	    return Err(Error::error(
		*line_number,
		String::from("Unexpected character"),
	    ));
	}
    };

    Ok(ScanResult::Tok(token))
}

fn identifier<I: Iterator<Item = char>>(
    chars: &mut Peekable<I>,
    line_number: usize,
    first: char
) -> Token {
    let mut identifier = String::from(first);
    let rest: String = chars.take_while(|c| c.is_alphanumeric()).collect();
    identifier.push_str(&rest);

    Token::new(TokenType::string_to_keyword_or_id(&identifier), line_number)
}

fn matches<I: Iterator<Item = char>>(chars: &mut Peekable<I>, expected: char) -> bool {
    if chars.peek() == Some(&expected) {
        chars.next();
        return true;
    }
    false
}

fn string<I: Iterator<Item = char>>(
    chars: &mut Peekable<I>,
    line_number: &mut usize,
) -> Result<String, Error> {
    let mut string = String::new();

    loop {
        match chars.next() {
	    None => {
                return Err(Error::error(
		    *line_number,
		    String::from("Unterminated string."),
                ))
	    }
	    Some('"') => break,
	    Some('\n') => *line_number += 1,
	    Some(character) => string.push(character),
        }
    }

    Ok(string)
}

fn number<I: Iterator<Item = char>>(chars: &mut Peekable<I>, beginning: char) -> Result<f64, Error> {
    println!("{beginning}");
    let mut number = String::from(beginning);
    let whole: String = chars.take_while(|c| c.is_numeric() || *c == '.').collect();
    number.push_str(&whole);

    Ok(number.parse::<f64>().unwrap())

}
