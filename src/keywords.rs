#[derive(Debug)]
pub enum Error {
    IncorrectIdentifier(Location),
    IncorrectKeyword(Location),
    IncorrectOperator(Location),
}

impl Error {
    pub fn to_string(self) -> String {
        let string = match self {
            Error::IncorrectIdentifier(e) => format!("Identifier err: {}, {}, {}", e.line, e.column, e.char).to_owned(),
            Error::IncorrectKeyword(e) => format!("Keyword err: {}, {}, {}", e.line, e.column, e.char).to_owned(),
            Error::IncorrectOperator(e) => format!("Operator err: {}, {}, {}", e.line, e.column, e.char).to_owned(),
        };
        string
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub enum TokenType {
    Keyword = 10,
    Identifier = 20,
    Operator = 30,
    ConstValue = 40,
    StringLiteral = 50,
    Separator = 60,
}

#[derive(Debug)]
enum State {
    Character(char),
    Letter(char),
    ValueCharacter(char),
    StringLiteralCharacter(char),
    IdentifierCharacter(char),
    Whitespace,
    IncludeI,
    IncludeN,
    IncludeC,
    IncludeL,
    IncludeU,
    IncludeD,
    IncludeE,
    IntN,
    IntT,
    ReturnE,
    ReturnT,
    ReturnU,
    ReturnR,
    ReturnN,
    AutoU,
    AutoT,
    AutoO,
    BoolO,
    BoolO2,
    BoolL,
    BreakR,
    BreakE,
    BreakA,
    BreakK,
    CaseA,
    CaseS,
    CaseE,
    CatchA,
    CatchT,
    CatchC,
    CatchH,
    CharH,
    CharA,
    CharR,
    ClassL,
    ClassA,
    ClassS,
    ClassS2,
    ConstO,
    ConstN,
    ConstS,
    ConstT,
    DefaultE,
    DefaultF,
    DefaultA,
    DefaultU,
    DefaultL,
    DefaultT,
    DeleteE,
    DeleteL,
    DeleteE2,
    DeleteT,
    DeleteE3,
    DoO,
    DoubleO,
    DoubleU,
    DoubleB,
    DoubleL,
    DoubleE,
    ElseL,
    ElseS,
    ElseE,
    FalseA,
    FalseL,
    FalseS,
    FalseE,
    FloatL,
    FloatO,
    FloatA,
    FloatT,
    ForO,
    ForR,
    FriendR,
    FriendI,
    FriendE,
    FriendN,
    FriendD,
    GotoO,
    GotoT,
    GotoO2,
    IfF,
    LongO,
    LongN,
    LongG,
    NamespaceA,
    NamespaceM,
    NamespaceE,
    NamespaceS,
    NamespaceP,
    NamespaceA2,
    NamespaceC,
    NamespaceE2,
    NewE,
    NewW,
    OperatorP,
    OperatorE,
    OperatorR,
    OperatorA,
    OperatorT,
    OperatorO,
    OperatorR2,
    ShortH,
    ShortO,
    ShortR,
    ShortT,
    SignedI,
    SignedG,
    SignedN,
    SignedE,
    SignedD,
    SizeofI,
    SizeofZ,
    SizeofE,
    SizeofO,
    SizeofF,
    StaticT,
    StaticA,
    StaticT2,
    StaticI,
    StaticC,
    StructT,
    StructR,
    StructU,
    StructC,
    StructT2,
    TemplateE,
    TemplateM,
    TemplateP,
    TemplateL,
    TemplateA,
    TemplateT,
    TemplateE2,
    ThisH,
    ThisI,
    ThisS,
    ThrowH,
    ThrowR,
    ThrowO,
    ThrowW,
    TrueR,
    TrueU,
    TrueE,
    TryR,
    TryY,
    UnionN,
    UnionI,
    UnionO,
    UnionN2,
    UnsignedN,
    UnsignedS,
    UnsignedI,
    UnsignedG,
    UnsignedN2,
    UnsignedE,
    UnsignedD,
    UsingS,
    UsingI,
    UsingN,
    UsingG,
    VirtualI,
    VirtualR,
    VirtualT,
    VirtualU,
    VirtualA,
    VirtualL,
    VoidO,
    VoidI,
    VoidD,
    WhileH,
    WhileI,
    WhileL,
    WhileE,
}

#[derive(Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub char: char,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token: String,
}

pub fn count_tokens(text: String) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut state: State = State::Whitespace;
    let mut location: Location = Location {
        line: 0,
        column: 0,
        char: ' ',
    };
    let mut current_idx: usize = 0;
    let mut current: char;
    let mut temporary_text = String::new();
    while current_idx < text.chars().count() {
        current = text.as_str().as_bytes()[current_idx] as char;
        location.char = current;
        print!("{current}");
        if current == '\n' {
            location.line += 1;
            location.column += 1;
        }
        current_idx += 1;
        match state {
            State::Whitespace => {
                state = match current {
                    ' ' | '\t' => State::Whitespace,
                    c if c.is_alphabetic() => {
                        temporary_text.push(c);
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        temporary_text.push(c);
                        State::ValueCharacter(c)
                    }
                    c if [';', '(', ')', ',', '{', '}'].contains(&c) => {
                        tokens.push(Token {
                            token: c.to_string(),
                            token_type: TokenType::Separator,
                        });
                        State::Character(c)
                    }
                    c => State::Character(c),
                };
            }
            State::Character('#') => {
                state = match current {
                    'i' => {
                        temporary_text.push_str("#i");
                        State::IncludeI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                }
            }
            State::Character('-') => {
                state = match current {
                    '-' => {
                        tokens.push(Token {
                            token: "--".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Character(current)
                    }
                    '=' => {
                        tokens.push(Token {
                            token: "-=".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Character(current)
                    }
                    c if c.is_alphabetic() => {
                        tokens.push(Token {
                            token: "-".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        tokens.push(Token {
                            token: "-".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::ValueCharacter(c)
                    }
                    c if c.is_whitespace() => {
                        tokens.push(Token {
                            token: "+".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Whitespace
                    }
                    c => State::Character(c),
                }
            }
            State::Character('+') => {
                state = match current {
                    '+' => {
                        tokens.push(Token {
                            token: "++".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Character(current)
                    }
                    '=' => {
                        tokens.push(Token {
                            token: "+=".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Character(current)
                    }
                    c if c.is_alphabetic() => {
                        tokens.push(Token {
                            token: "+".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        tokens.push(Token {
                            token: "+".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::ValueCharacter(c)
                    }
                    c if c.is_whitespace() => {
                        tokens.push(Token {
                            token: "+".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Whitespace
                    }
                    c => State::Character(c),
                }
            }
            State::Character('<') => {
                state = match current {
                    '=' => {
                        tokens.push(Token {
                            token: "<=".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Character(current)
                    }
                    c if c.is_alphabetic() => {
                        tokens.push(Token {
                            token: "+".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        tokens.push(Token {
                            token: "<".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::ValueCharacter(c)
                    }
                    c if c.is_whitespace() => {
                        tokens.push(Token {
                            token: "<".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Whitespace
                    }
                    c => State::Character(c),
                }
            }
            State::Character('>') => {
                state = match current {
                    '=' => {
                        tokens.push(Token {
                            token: ">=".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Character(current)
                    }
                    c if c.is_alphabetic() => {
                        tokens.push(Token {
                            token: ">".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        tokens.push(Token {
                            token: ">".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::ValueCharacter(c)
                    }
                    c if c.is_whitespace() => {
                        tokens.push(Token {
                            token: ">".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Whitespace
                    }
                    c => State::Character(c),
                }
            }
            State::Character('=') => {
                state = match current {
                    '=' => {
                        tokens.push(Token {
                            token: "==".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Character(current)
                    }
                    c if c.is_alphabetic() => {
                        tokens.push(Token {
                            token: "=".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        tokens.push(Token {
                            token: "=".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::ValueCharacter(c)
                    }
                    c if c.is_whitespace() => {
                        tokens.push(Token {
                            token: "=".to_owned(),
                            token_type: TokenType::Operator,
                        });
                        State::Whitespace
                    }
                    c => State::Character(c),
                }
            }
            State::Character('"') => {
                state = match current {
                    c => {
                        temporary_text.push(c);
                        State::StringLiteralCharacter(c)
                    }
                }
            }
            State::Character(c) => {
                state = match current {
                    c if c.is_alphabetic() => {
                        temporary_text.push(c);
                        State::Letter(c)
                    }
                    '#' => {
                        temporary_text.push(c);
                        State::Character('#')
                    }
                    c if c.is_numeric() || c == '.' => {
                        temporary_text.push(c);
                        State::ValueCharacter(c)
                    }
                    c if [' ', '\t', ';', '(', ')', ',', '{', '}'].contains(&c) => match c {
                        ' ' | '\t' => State::Whitespace,
                        _ => {
                            tokens.push(Token {
                                token: c.to_string(),
                                token_type: TokenType::Separator,
                            });
                            State::Character(c)
                        }
                    },
                    c => State::Character(c),
                }
            }
            State::StringLiteralCharacter(_c) => {
                state = match current {
                    c if c.is_alphabetic() => {
                        temporary_text.push(c);
                        State::StringLiteralCharacter(c)
                    }
                    '"' => {
                        tokens.push(Token {
                            token_type: TokenType::StringLiteral,
                            token: temporary_text.clone(),
                        });
                        temporary_text.clear();
                        State::Whitespace
                    }
                    c => {
                        temporary_text.push(c);
                        State::StringLiteralCharacter(c)
                    }
                }
            }
            State::ValueCharacter(_c) => {
                state = match current {
                    c if c.is_numeric() || c == '.' => {
                        temporary_text.push(c);
                        State::ValueCharacter(c)
                    }
                    c if [' ', '\t', ';', '(', ')', ',', '{', '}'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::ConstValue,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            _ => {
                                tokens.push(Token {
                                    token: c.to_string(),
                                    token_type: TokenType::Separator,
                                });
                                State::Character(c)
                            }
                        }
                    }
                    c => State::Character(c),
                }
            }
            State::IdentifierCharacter(_c) => {
                state = match current {
                    c if [' ', '\t', ';', '(', ')', ',', '{', '}', '\n'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            _ => {
                                tokens.push(Token {
                                    token: c.to_string(),
                                    token_type: TokenType::Separator,
                                });
                                State::Character(c)
                            }
                        }
                    }
                    c if c.is_alphanumeric() || c == '_' => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _c => return Err(Error::IncorrectIdentifier(location)),
                }
            }
            State::IncludeI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::IncludeN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::IncludeN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'c' => {
                        temporary_text.push('c');
                        State::IncludeC
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::IncludeC => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::IncludeL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::IncludeL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::IncludeU
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::IncludeU => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'd' => {
                        temporary_text.push('d');
                        State::IncludeD
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::IncludeD => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::IncludeE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::IncludeE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }

            // `int` | `inline`
            State::Letter('i') => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::IntN
                    }
                    'f' => {
                        temporary_text.push('f');
                        State::IfF
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::IntN => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::IntT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::IntT => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            // `return`
            State::Letter('r') => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::ReturnE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::ReturnE => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::ReturnT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ReturnT => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::ReturnU
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ReturnU => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::ReturnR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ReturnR => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::ReturnN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ReturnN => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            // `auto`
            State::Letter('a') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::AutoU
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::AutoU => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::AutoT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::AutoT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::AutoO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::AutoO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('b') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::BoolO
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::BreakR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::BoolO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::BoolO2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::BoolO2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::BoolL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::BoolL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::BreakR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::BreakE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::BreakE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::BreakA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::BreakA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'k' => {
                        temporary_text.push('k');
                        State::BreakK
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::BreakK => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('c') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::CaseA
                    }
                    'h' => {
                        temporary_text.push('h');
                        State::CharH
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::ClassL
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::ConstO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::CaseA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::CaseS
                    }
                    't' => {
                        temporary_text.push('t');
                        State::CatchT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CaseS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::CaseE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CaseE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CatchT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'c' => {
                        temporary_text.push('c');
                        State::CatchC
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CatchC => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::CatchA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CatchH => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CharH => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::CharA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CharA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::CharR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::CharR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ClassL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::ClassA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ClassA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::ClassS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ClassS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::ClassS2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ClassS2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ConstO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::ConstN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ConstN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::ConstS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ConstS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::ConstT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ConstT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('d') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::DefaultE
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::DoO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::DefaultE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'f' => {
                        temporary_text.push('f');
                        State::DefaultF
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::DeleteL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DefaultF => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::DefaultA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DefaultA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::DefaultU
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DefaultU => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::DefaultL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DefaultL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::DefaultT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DefaultT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DeleteL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::DeleteE2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DeleteE2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::DeleteT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DeleteT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::DeleteE3
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DeleteE3 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DoO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::DoubleU
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DoubleU => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'b' => {
                        temporary_text.push('b');
                        State::DoubleB
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DoubleB => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::DoubleL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DoubleL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::DoubleE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::DoubleE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('e') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::ElseL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::ElseL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::ElseS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ElseS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::ElseE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ElseE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('f') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::FalseA
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::ForO
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::FloatL
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::FriendR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::FalseA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::FalseL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FalseL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::FalseS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FalseS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::FalseE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FalseE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FloatL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::FloatO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FloatO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::FloatA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FloatA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::FloatT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FloatT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ForO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::ForR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ForR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FriendR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::FriendI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FriendI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::FriendE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FriendE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::FriendN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FriendN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'd' => {
                        temporary_text.push('d');
                        State::FriendD
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::FriendD => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('g') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::GotoO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::GotoO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::GotoT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::GotoT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::GotoO2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::GotoO2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::IfF => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('l') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::LongO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::LongO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::LongN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::LongN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'g' => {
                        temporary_text.push('g');
                        State::LongG
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::LongG => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('n') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::NamespaceA
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::NewE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::NamespaceA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'm' => {
                        temporary_text.push('m');
                        State::NamespaceM
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NamespaceM => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::NamespaceE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NamespaceE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::NamespaceS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NamespaceS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'p' => {
                        temporary_text.push('p');
                        State::NamespaceP
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NamespaceP => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::NamespaceA2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NamespaceA2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'c' => {
                        temporary_text.push('c');
                        State::NamespaceC
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NamespaceC => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::NamespaceE2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NamespaceE2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NewE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'w' => {
                        temporary_text.push('w');
                        State::NewW
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::NewW => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('o') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'p' => {
                        temporary_text.push('p');
                        State::OperatorP
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::OperatorP => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::OperatorE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::OperatorE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::OperatorR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::OperatorR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::OperatorA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::OperatorA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::OperatorT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::OperatorT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::OperatorO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::OperatorO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::OperatorR2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::OperatorR2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('s') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'h' => {
                        temporary_text.push('h');
                        State::ShortH
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::SignedI
                    }
                    't' => {
                        temporary_text.push('t');
                        State::StaticT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::ShortH => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::ShortO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ShortO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::ShortR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ShortR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::ShortT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ShortT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SignedI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'g' => {
                        temporary_text.push('g');
                        State::SignedG
                    }
                    'z' => {
                        temporary_text.push('z');
                        State::SizeofZ
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SignedG => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::SignedN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SignedN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::SignedE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SignedE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'd' => {
                        temporary_text.push('d');
                        State::SignedD
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SignedD => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SizeofI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'z' => {
                        temporary_text.push('z');
                        State::SizeofZ
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SizeofZ => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::SizeofE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SizeofE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::SizeofO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SizeofO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'f' => {
                        temporary_text.push('f');
                        State::SizeofF
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::SizeofF => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StaticT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::StaticA
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::StructR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StaticA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::StaticT2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StaticT2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::StaticI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StaticI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'c' => {
                        temporary_text.push('c');
                        State::StaticC
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StaticC => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StructR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::StructU
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StructU => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'c' => {
                        temporary_text.push('c');
                        State::StructC
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StructC => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::StructT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::StructT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('t') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::TemplateE
                    }
                    'h' => {
                        temporary_text.push('h');
                        State::ThrowH
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::TrueR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::TemplateE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'm' => {
                        temporary_text.push('m');
                        State::TemplateM
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TemplateM => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'p' => {
                        temporary_text.push('p');
                        State::TemplateP
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TemplateP => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::TemplateL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TemplateL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::TemplateA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TemplateA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::TemplateT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TemplateT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::TemplateE2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TemplateE2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ThisH => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::ThisI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ThisI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    's' => {
                        temporary_text.push('s');
                        State::ThisS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ThisS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ThrowH => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::ThrowR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ThrowR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::ThrowO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ThrowO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'w' => {
                        temporary_text.push('w');
                        State::ThrowW
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::ThrowW => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TrueR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::TrueU
                    }
                    'y' => {
                        temporary_text.push('y');
                        State::TryY
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TrueU => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::TrueE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TrueE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::TryY => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('u') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::UnionN
                    }
                    's' => {
                        temporary_text.push('s');
                        State::UsingS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::UnionN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::UnionI
                    }
                    's' => {
                        temporary_text.push('s');
                        State::UnsignedS
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnionI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::UnionO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnionO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::UnionN2
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnionN2 => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnsignedS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::UnsignedI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnsignedI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'g' => {
                        temporary_text.push('g');
                        State::UnsignedG
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnsignedG => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::UnsignedN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnsignedN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::UnsignedE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnsignedE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'd' => {
                        temporary_text.push('d');
                        State::UnsignedD
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UnsignedD => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UsingS => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::UsingI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UsingI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'n' => {
                        temporary_text.push('n');
                        State::UsingN
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UsingN => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'g' => {
                        temporary_text.push('g');
                        State::UsingG
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::UsingG => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('v') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::VirtualI
                    }
                    'o' => {
                        temporary_text.push('o');
                        State::VoidO
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::VirtualI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'r' => {
                        temporary_text.push('r');
                        State::VirtualR
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VirtualR => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    't' => {
                        temporary_text.push('t');
                        State::VirtualT
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VirtualT => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'u' => {
                        temporary_text.push('u');
                        State::VirtualU
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VirtualU => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'a' => {
                        temporary_text.push('a');
                        State::VirtualA
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VirtualA => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::VirtualL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VirtualL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VoidO => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::VoidI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VoidI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'd' => {
                        temporary_text.push('d');
                        State::VoidD
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::VoidD => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::Letter('w') => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'h' => {
                        temporary_text.push('h');
                        State::WhileH
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    c => State::Character(c),
                };
            }
            State::WhileH => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'i' => {
                        temporary_text.push('i');
                        State::WhileI
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::WhileI => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'l' => {
                        temporary_text.push('l');
                        State::WhileL
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::WhileL => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    'e' => {
                        temporary_text.push('e');
                        State::WhileE
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::WhileE => {
                state = match current {
                    c if [' ', '\t', ';'].contains(&c) => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Keyword,
                        });
                        temporary_text.clear();
                        match c {
                            c if c.is_whitespace() => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("!!!"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }

            State::Letter(_l) => {
                state = match current {
                    c if c == ' ' || c == '\t' || c == ';' => {
                        tokens.push(Token {
                            token: temporary_text.clone(),
                            token_type: TokenType::Identifier,
                        });
                        temporary_text.clear();
                        match c {
                            ' ' | '\t' => State::Whitespace,
                            ';' => State::Character(';'),
                            _ => panic!("???"),
                        }
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push(c);
                        State::IdentifierCharacter(c)
                    }
                    _ => return Err(Error::IncorrectIdentifier(location)),
                };
            }
            _ => {}
        }
    }
    Ok(tokens)
}
