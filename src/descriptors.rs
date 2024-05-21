use std::collections::{BTreeSet};
use crate::keywords::{Token, TokenType};

pub struct Table {
    pub descriptors: String,
    pub pseudocode: String,
    pub identifiers: String,
    pub keywords: String,
    pub separators: String,
    pub strings: String,
    pub consts: String,
    pub operators: String,
}

pub fn create_descriptors(tokens: Vec<Token>) -> Option<Table> {
    let clone = tokens.clone();
    let (identifiers, rest): (Vec<Token>, Vec<Token>) = clone.into_iter().partition(|x| x.token_type == TokenType::Identifier);
    let (keywords, rest): (Vec<Token>, Vec<Token>) = rest.into_iter().partition(|x| x.token_type == TokenType::Keyword);
    let (separators, rest): (Vec<Token>, Vec<Token>) = rest.into_iter().partition(|x| x.token_type == TokenType::Separator);
    let (strings, rest): (Vec<Token>, Vec<Token>) = rest.into_iter().partition(|x| x.token_type == TokenType::StringLiteral);
    let (consts, operators): (Vec<Token>, Vec<Token>) = rest.into_iter().partition(|x| x.token_type == TokenType::ConstValue);

    let mut identifier_set: BTreeSet<String> = BTreeSet::new();
    for i in identifiers {
        identifier_set.insert(i.token);
    }
    let mut keywords_set: BTreeSet<String> = BTreeSet::new();
    for i in keywords {
        keywords_set.insert(i.token);
    }
    let mut separators_set: BTreeSet<String> = BTreeSet::new();
    for i in separators {
        separators_set.insert(i.token);
    }
    let mut strings_set: BTreeSet<String> = BTreeSet::new();
    for i in strings {
        strings_set.insert(i.token);
    }
    let mut consts_set: BTreeSet<String> = BTreeSet::new();
    for i in consts {
        consts_set.insert(i.token);
    }
    let mut operators_set: BTreeSet<String> = BTreeSet::new();
    for i in operators {
        operators_set.insert(i.token);
    }

    let descriptors: String = tokens.iter().map(|x| {
        let x = x.clone();
        match x.token_type {
            TokenType::Keyword => format!("({},{})", x.token_type as u8, keywords_set.clone().iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::Identifier => format!("({},{})", x.token_type as u8, identifier_set.clone().iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::Operator => format!("({},{})", x.token_type as u8, operators_set.clone().iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::ConstValue => format!("({},{})", x.token_type as u8, consts_set.clone().iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::StringLiteral => format!("({},{})", x.token_type as u8, strings_set.clone().iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::Separator => format!("({},{})", x.token_type as u8, separators_set.clone().iter().position(|y| y == &x.token).unwrap().to_string()),
        }
    }).collect();
    let pseudocode: String = tokens.iter().map(|x| {
        let x = x.clone();
        match x.token_type {
            TokenType::Keyword => format!("{} ", x.token),
            TokenType::Identifier => format!("id{} ", identifier_set.iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::Operator => format!("{}", x.token),
            TokenType::ConstValue => format!("const{} ", consts_set.iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::StringLiteral => format!("str{} ", strings_set.iter().position(|y| y == &x.token).unwrap().to_string()),
            TokenType::Separator => format!("{}", x.token),
        }
    }).collect();

    let identifiers = identifier_set.iter().enumerate()
        .map(|(idx, x) | format!("\n{} {}", idx, x))
        .collect();
    let keywords = keywords_set.iter().enumerate()
        .map(|(idx, x) | format!("\n{} {}", idx, x))
        .collect();
    let separators = separators_set.iter().enumerate()
        .map(|(idx, x) | format!("\n{} {}", idx, x))
        .collect();
    let strings = strings_set.iter().enumerate()
        .map(|(idx, x) | format!("\n{} {}", idx, x))
        .collect();
    let consts = consts_set.iter().enumerate()
        .map(|(idx, x) | format!("\n{} {}", idx, x))
        .collect();
    let operators = operators_set.iter().enumerate()
        .map(|(idx, x) | format!("\n{} {}", idx, x))
        .collect();



    Some(Table {
        descriptors,
        pseudocode,
        identifiers,
        keywords,
        separators,
        strings,
        consts,
        operators,
    })

}