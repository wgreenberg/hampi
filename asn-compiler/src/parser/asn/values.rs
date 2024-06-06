//! Functions related to parsing of various Values

use crate::tokenizer::{types::TokenType, Token};
use anyhow::Result;

use crate::parser::utils::{expect_one_of_tokens, parse_set_ish_value};

// Parses a given set of 'tokens' as a value and returns a string corresponding to one that would
// be generated by concatenating those tokens. Note: It should be possible to regenerate, original
// tokens, by 'tokenize'ing the string.
pub(crate) fn parse_value(tokens: &[Token]) -> Result<(String, usize)> {
    if !expect_one_of_tokens(
        tokens,
        &[
            Token::is_identifier,
            Token::is_numeric,
            Token::is_bitstring,
            Token::is_hexstring,
            Token::is_tstring,
            Token::is_curly_begin,
            Token::is_round_begin,
        ],
    )? {
        Err(unexpected_token!(
            "'IDENTIFIER', 'NUMBER', 'Bit String', 'Hex String', 'String', '{', '('",
            tokens[0]
        )
        .into())
    } else {
        let token = &tokens[0];
        match token.r#type {
            TokenType::Identifier
            | TokenType::NumberInt
            | TokenType::BitString
            | TokenType::HexString
            | TokenType::TString => Ok((token.text.clone(), 1)),
            _ => parse_set_ish_value(tokens),
        }
    }
}

// TODO: Add Test cases
