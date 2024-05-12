use std::collections::HashSet;
use crate::keywords::Error::IncorrectIdentifier;
use crate::keywords::State::Or_eq_;

enum State {
    Start,
    Whitespace,
    Newline,
    Letter(char),
    Character(char),
    Number(char),
    // Semicolon ends a declaration
    Semicolon,
    // Whitespace between keyword and identifier
    WhitespaceIdentifier,
    // comparison
    LT,
    LE,
    GT,
    GE,
    NE,
    Equal,
    // bitshifts
    Shl,
    Shr,
    // operator + assign
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ShrAssign,
    ShlAssign,
    // `#include`
    IncludeI,
    IncludeN,
    IncludeC,
    IncludeL,
    IncludeU,
    IncludeD,
    IncludeE,
    IncludeWhitespace,
    IncludeIdentifier,
    // `auto`
    AutoU,
    AutoT,
    AutoO,
    AutoWhitespace,
    // `bit` in `bitand` | `bitor`
    BitI,
    BitT,
    // `bitand`
    BitandA,
    BitandN,
    BitandD,
    BitandWhitespace,
    // `bitor`
    BitorO,
    BitorR,
    BitorWhitespace,
    // `bool`
    BoolO,
    BoolL,
    BoolWhitespace,
    // `break`
    BreakR,
    BreakE,
    BreakA,
    BreakK,
    BreakSemicolon,
    // `ca` in `case` | `catch`
    CaA,
    // `case`
    CaseS,
    CaseE,
    // `catch`
    CatchT,
    CatchC,
    CatchH,
    CatchWhitespace,
    // `char`
    CharH,
    CharA,
    CharR,
    CharWhitespace,
    // `class`
    ClassL,
    ClassA,
    ClassS,
    ClassWhitespace,
    // `const`
    ConstO,
    ConstN,
    ConstS,
    ConstT,
    ConstWhitespace,
    // `de` in `default` | `delete`
    DeE,
    // `default`
    DefaultF,
    DefaultA,
    DefaultU,
    DefaultL,
    DefaultT,
    DefaultWhitespace,
    // `delete`
    DeleteL,
    DeleteT,
    // `do`
    DoO,
    DoWhitespace,
    // `double`
    DoubleU,
    DoubleB,
    DoubleL,
    DoubleE,
    DoubleWhitespace,
    // `else`
    ElseL,
    ElseS,
    ElseE,
    ElseWhitespace,
    // `enum`
    EnumN,
    EnumU,
    EnumM,
    // `false`
    FalseA,
    FalseL,
    FalseS,
    FalseE,
    FalseWhitespace,
    // `float`
    FloatL,
    FloatO,
    FloatA,
    FloatT,
    // `for`
    ForO,
    ForR,
    ForWhitespace,
    // `friend`
    FriendR,
    FriendI,
    FriendE,
    FriendN,
    FriendD,
    // `goto`
    GotoO,
    GotoT,
    GotoWhitespace,
    // `if`
    IfF,
    IfWhitespace,
    // `in` in `inline` | `int`
    InN,
    // `inline`
    InlineL,
    InlineI,
    InlineE,
    InlineWhitespace,
    // `int`
    IntT,
    // `long`
    LongO,
    LongN,
    LongG,
    // `mutable`
    MutableU,
    MutableT,
    MutableA,
    MutableB,
    MutableL,
    MutableE,
    MutableWhitespace,
    // `namespace`
    NamespaceA,
    NamespaceM,
    NamespaceE,
    NamespaceS,
    NamespaceP,
    NamespaceC,
    NamespaceWhitespace,
    // `new`
    NewE,
    NewW,
    NewWhitespace,
    // `not`
    NotO,
    NotT,
    NotWhitespace,
    // `not_eq`
    Not_eq_,
    Not_eqE,
    Not_eqQ,
    Not_eqWhitespace,
    // `nullptr`
    NullptrU,
    NullptrL,
    NullptrL2,
    NullptrP,
    NullptrT,
    NullptrR,
    NullptrWhitespace,
    // `operator`
    OperatorP,
    OperatorE,
    OperatorR,
    OperatorA,
    OperatorT,
    OperatorO,
    OperatorWhitespace,
    // `or`
    OrR,
    OrWhitespace,
    // `or_eq`
    Or_eq_,
    Or_eqE,
    Or_eqQ,
    Or_eqWhitespace,
    // `pr` in `private` | `protected`
    PrR,
    // `private`
    PrivateI,
    PrivateV,
    PrivateA,
    PrivateT,
    PrivateE,
    PrivateColon,
    // `protected`
    ProtectedO,
    ProtectedT,
    ProtectedE,
    ProtectedC,
    ProtectedD,
    ProtectedColon,
    // `public`
    PublicU,
    PublicB,
    PublicL,
    PublicI,
    PublicC,
    PublicColon,
    // `return`
    ReturnE,
    ReturnT,
    ReturnU,
    ReturnR,
    ReturnN,
    // `short`
    ShortH,
    ShortO,
    ShortR,
    ShortT,
    // `si` in `signed` | `sizeof`
    SiI,
    // `signed`
    SignedG,
    SignedN,
    SignedE,
    SignedD,
    SignedWhitespace,
    // `sizeof`
    SizeofZ,
    SizeofE,
    SizeofO,
    SizeofF,
    SizeofWhitespace,
    // `st` in `static` | `struct`
    StT,
    // `static`
    StaticA,
    StaticI,
    StaticC,
    StaticWhitespace,
    // `struct`
    StructR,
    StructU,
    StructC,
    // `switch`
    SwitchW,
    SwitchI,
    SwitchT,
    SwitchC,
    SwitchH,
    SwitchWhitespace,
    // `template`
    TemplateE,
    TemplateM,
    TemplateP,
    TemplateL,
    TemplateA,
    TemplateT,
    TemplateWhitespace,
    // `th` in `this` | `throw`
    ThH,
    // `this`
    ThisI,
    ThisS,
    ThisWhitespace,
    // `throw`
    ThrowR,
    ThrowO,
    ThrowW,
    ThrowWhitespace,
    // `tr` in `true` | `try`
    TrR,
    // `true`
    TrueU,
    TrueE,
    TrueWhitespace,
    // `try`
    TryY,
    TryWhitespace,
    // `un` in `union` | `unsigned`
    UnN,
    // `union`
    UnionI,
    UnionO,
    UnionWhitespace,
    // `unsigned`
    UnsignedS,
    UnsignedI,
    UnsignedG,
    UnsignedE,
    UnsignedD,
    UnsignedWhitespace,
    // `using`
    UsingS,
    UsingI,
    UsingN,
    UsingG,
    UsingWhitespace,
    // `virtual`
    VirtualI,
    VirtualR,
    VirtualT,
    VirtualU,
    VirtualA,
    VirtualL,
    VirtualWhitespace,
    // `void`
    VoidO,
    VoidI,
    VoidD,
    // `while`
    WhileH,
    WhileI,
    WhileL,
    WhileE,
    WhileWhitespace,
    DeleteE,
    DeleteE2,
    InlineN,
    ProtectedT2,
    ProtectedE2,
    StaticT,
    StructT,
    TemplateE2,
    UnionN,
    UnsignedN,
    BoolO2,
    ClassS2,
    GotoO2,
    NamespaceA2,
    NamespaceE2,
    OperatorR2,
}
#[derive(Debug)]
pub enum Error {
    VariableNotDefined,
    NotInclude,
    IncorrectIdentifier,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Value,
    Punctuation,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token: String,
}

pub fn get_keywords(text: String) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut identifier: HashSet<String> = HashSet::new();
    let mut state: State = State::Start;
    let mut current_idx: usize = 0;
    let mut current: char;
    let mut temporary_text = String::new();
    while current_idx < text.chars().count() {
        current = text.as_str().as_bytes()[current_idx] as char;
        match state {
            State::Start | State::Newline => {
                state = match current {
                    ' ' | '\t' => State::Whitespace,
                    '\n' => State::Newline,
                    c if c.is_alphabetic() => State::Letter(c),
                    c if c.is_numeric() => State::Number(c),
                    c => State::Character(c),
                };
                current_idx += 1;
            },
            // `#include`
            State::Character('#') => {
                state = match current {
                    'i' => {
                        temporary_text.push('#');
                        temporary_text.push(current);
                        State::IncludeI
                    },
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeI => {
                state = match current {
                    'n' => {
                        temporary_text.push(current);
                        State::IncludeN
                    }
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeN => {
                state = match current {
                    'c' => {
                        temporary_text.push(current);
                        State::IncludeC
                    }
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeC => {
                state = match current {
                    'l' => {
                        temporary_text.push(current);
                        State::IncludeL
                    }
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeL => {
                state = match current {
                    'u' => {
                        temporary_text.push(current);
                        State::IncludeU
                    }
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeU => {
                state = match current {
                    'd' => {
                        temporary_text.push(current);
                        State::IncludeD
                    }
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeD => {
                state = match current {
                    'e' => {
                        temporary_text.push(current);
                        State::IncludeE
                    }
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: temporary_text.clone(),
                        });
                        temporary_text.clear();
                        State::IncludeWhitespace
                    },
                    '\n' => State::Newline,
                    _ => return Err(Error::NotInclude),
                };
                current_idx += 1;
            }
            State::IncludeWhitespace => {
                state = match current {
                    '<' | '"' => {
                        temporary_text.push(current);
                        State::IncludeIdentifier
                    }
                    _ => return Err(Error::NotInclude),
                }
            }
            State::IncludeIdentifier => {
                state = match current {
                    '>' | '"' => {
                        temporary_text.push(current);
                        tokens.push(Token {
                            token_type: TokenType::Identifier,
                            token: temporary_text.clone(),
                        });
                        temporary_text.clear();
                        State::Start
                    }
                    _ => {
                        temporary_text.push(current);
                        State::IncludeIdentifier
                    }
                };
                current_idx += 1;
            }
            // `auto`
            State::Character('a') => {
                state = match current {
                    'u' => State::AutoU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('a');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::AutoU => {
                state = match current {
                    't' => State::AutoT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("au");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::AutoT => {
                state = match current {
                    'o' => State::AutoO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("aut");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::AutoO => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "auto".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("auto");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `bitand`
            State::Character('b') => {
                state = match current {
                    'i' => State::BitI,
                    'o' => State::BoolO,
                    'r' => State::BreakR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('b');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BitI => {
                state = match current {
                    't' => State::BitT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BitT => {
                state = match current {
                    'a' => State::BitandA,
                    'o' => State::BitorO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bit");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BitandA => {
                state = match current {
                    'n' => State::BitandN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bita");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BitandN => {
                state = match current {
                    'd' => State::BitandD,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bitan");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BitandD => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "bitand".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bitand");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `bitor`
            State::BitorO => {
                state = match current {
                    'r' => State::BitorR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bito");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BitorR => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "bitor".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bitor");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `bool`
            State::BoolO => {
                state = match current {
                    'o' => State::BoolO2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bo");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BoolO2 => {
                state = match current {
                    'l' => State::BoolL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bo");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BoolL => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "bool".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bool");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `break`
            State::BreakR => {
                state = match current {
                    'e' => State::BreakE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("br");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BreakE => {
                state = match current {
                    'a' => State::BreakA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("bre");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BreakA => {
                state = match current {
                    'k' => State::BreakK,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("brea");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::BreakK => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "break".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("break");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `case`
            State::Character('c') => {
                state = match current {
                    'a' => State::CaA,
                    'h' => State::CharH,
                    'l' => State::ClassS,
                    'o' => State::ConstO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('c');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::CaA => {
                state = match current {
                    's' => State::CaseS,
                    't' => State::CatchT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("ca");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::CaseS => {
                state = match current {
                    'e' => State::CaseE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("cas");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::CaseE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "case".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("case");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `catch`
            State::CatchT => {
                state = match current {
                    'c' => State::CatchC,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("cat");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::CatchC => {
                state = match current {
                    'h' => State::CatchH,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("catc");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::CatchH => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "catch".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("catch");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `char`
            State::CharH => {
                state = match current {
                    'a' => State::CharA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("ch");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::CharA => {
                state = match current {
                    'r' => State::CharR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("cha");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::CharR => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "char".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("char");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `class`
            State::ClassL => {
                state = match current {
                    'a' => State::ClassA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("cl");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ClassA => {
                state = match current {
                    's' => State::ClassS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("cla");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ClassS => {
                state = match current {
                    's' => State::ClassS2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("clas");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ClassS2 => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "class".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("class");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `const`
            State::ConstO => {
                state = match current {
                    'n' => State::ConstN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("co");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ConstN => {
                state = match current {
                    's' => State::ConstS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("con");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ConstS => {
                state = match current {
                    't' => State::ConstT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("cons");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ConstT => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "const".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("const");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `default`
            State::Character('d') => {
                state = match current {
                    'e' => State::DeE,
                    'o' => State::DoO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('d');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DeE => {
                state = match current {
                    'f' => State::DefaultF,
                    'l' => State::DeleteL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("de");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DefaultF => {
                state = match current {
                    'a' => State::DefaultA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("def");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DefaultA => {
                state = match current {
                    'u' => State::DefaultU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("defa");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DefaultU => {
                state = match current {
                    'l' => State::DefaultL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("defau");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DefaultL => {
                state = match current {
                    't' => State::DefaultT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("defaul");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DefaultT => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "default".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("default");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `delete`
            State::DeleteL => {
                state = match current {
                    'e' => State::DeleteE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("del");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DeleteE => {
                state = match current {
                    'l' => State::DeleteL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("de");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DeleteT => {
                state = match current {
                    'e' => State::DeleteE2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("delet");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DeleteE2 => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "delete".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("delete");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `do`
            State::DoO => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "do".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    'u' => State::DoubleU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("do");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `double`
            State::DoubleU => {
                state = match current {
                    'b' => State::DoubleB,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("dou");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DoubleB => {
                state = match current {
                    'l' => State::DoubleL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("doub");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DoubleL => {
                state = match current {
                    'e' => State::DoubleE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("doubl");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::DoubleE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "double".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("double");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `else`
            State::Character('e') => {
                state = match current {
                    'n' => State::EnumN,
                    'l' => State::ElseL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('e');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ElseL => {
                state = match current {
                    's' => State::ElseS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("el");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ElseS => {
                state = match current {
                    'e' => State::ElseE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("els");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ElseE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "else".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("else");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `enum`
            State::EnumN => {
                state = match current {
                    'u' => State::EnumU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("en");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::EnumU => {
                state = match current {
                    'm' => State::EnumM,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("enu");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::EnumM => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "enum".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("enum");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `false`
            State::Character('f') => {
                state = match current {
                    'a' => State::FalseA,
                    'l' => State::FloatL,
                    'o' => State::ForO,
                    'r' => State::FriendR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('f');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FalseA => {
                state = match current {
                    'l' => State::FalseL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("fa");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FalseL => {
                state = match current {
                    's' => State::FalseS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("fal");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FalseS => {
                state = match current {
                    'e' => State::FalseE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("fals");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FalseE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "false".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("false");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `float`
            State::FloatL => {
                state = match current {
                    'o' => State::FloatO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("fl");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FloatO => {
                state = match current {
                    'a' => State::FloatA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("flo");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FloatA => {
                state = match current {
                    't' => State::FloatT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("floa");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FloatT => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "float".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("float");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `for`
            State::ForO => {
                state = match current {
                    'r' => State::ForR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("fo");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ForR => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "for".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("for");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `friend`
            State::FriendR => {
                state = match current {
                    'i' => State::FriendI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("fr");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FriendI => {
                state = match current {
                    'e' => State::FriendE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("fri");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FriendE => {
                state = match current {
                    'n' => State::FriendN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("frie");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FriendN => {
                state = match current {
                    'd' => State::FriendD,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("frien");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::FriendD => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "friend".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("friend");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `goto`
            State::Character('g') => {
                state = match current {
                    'o' => State::GotoO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('g');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::GotoO => {
                state = match current {
                    't' => State::GotoT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("go");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::GotoT => {
                state = match current {
                    'o' => State::GotoO2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("got");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::GotoO2 => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "goto".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("goto");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `if`
            State::Character('i') => {
                state = match current {
                    'f' => State::IfF,
                    'n' => State::InN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('i');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::IfF => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "if".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("if");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `inline`
            State::InN => {
                state = match current {
                    'l' => State::InlineL,
                    'e' => State::InlineE,
                    't' => State::IntT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("in");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::InlineL => {
                state = match current {
                    'i' => State::InlineI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("inl");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::InlineI => {
                state = match current {
                    'n' => State::InlineN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("i");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::InlineE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "inline".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("inline");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `int`
            State::IntT => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "int".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("int");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `long`
            State::Character('l') => {
                state = match current {
                    'o' => State::LongO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('l');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::LongO => {
                state = match current {
                    'n' => State::LongN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("lo");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::LongN => {
                state = match current {
                    'g' => State::LongG,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("lon");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::LongG => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "long".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("long");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `mutable`
            State::Character('m') => {
                state = match current {
                    'u' => State::MutableU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('m');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::MutableU => {
                state = match current {
                    't' => State::MutableT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("mu");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::MutableT => {
                state = match current {
                    'a' => State::MutableA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("mut");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::MutableA => {
                state = match current {
                    'b' => State::MutableB,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("muta");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::MutableB => {
                state = match current {
                    'l' => State::MutableL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("mutab");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::MutableL => {
                state = match current {
                    'e' => State::MutableE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("mutabl");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::MutableE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "mutable".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("mutable");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `namespace`
            State::Character('n') => {
                state = match current {
                    'a' => State::NamespaceA,
                    'e' => State::NewE,
                    'o' => State::NotO,
                    'u' => State::NullptrU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('n');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceA => {
                state = match current {
                    'm' => State::NamespaceM,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("na");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceM => {
                state = match current {
                    'e' => State::NamespaceE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("nam");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceE => {
                state = match current {
                    's' => State::NamespaceS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("name");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceS => {
                state = match current {
                    'p' => State::NamespaceP,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("names");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceP => {
                state = match current {
                    'a' => State::NamespaceA2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("namesp");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceA2 => {
                state = match current {
                    'm' => State::NamespaceM,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("na");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceC => {
                state = match current {
                    'e' => State::NamespaceE2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("namespac");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NamespaceE2 => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "namespace".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("namespace");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `new`
            State::NewE => {
                state = match current {
                    'w' => State::NewW,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("ne");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NewW => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "new".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("new");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `not`
            State::NotO => {
                state = match current {
                    't' => State::NotT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("no");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NotT => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "not".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    '_' => State::Not_eq_,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("not");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `not_eq`
            State::Not_eq_ => {
                state = match current {
                    'e' => State::Not_eqE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("not_");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::Not_eqE => {
                state = match current {
                    'q' => State::Not_eqQ,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("not_e");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::Not_eqQ => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "not_eq".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("not_eq");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `nullptr`
            State::NullptrU => {
                state = match current {
                    'l' => State::NullptrL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("nu");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NullptrL => {
                state = match current {
                    'l' => State::NullptrL2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("nul");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NullptrL2 => {
                state = match current {
                    'l' => State::NullptrL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("nul");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NullptrP => {
                state = match current {
                    't' => State::NullptrT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("nullp");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NullptrT => {
                state = match current {
                    'r' => State::NullptrR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("nullpt");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::NullptrR => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "nullptr".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("nullptr");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `operator`
            State::Character('o') => {
                state = match current {
                    'p' => State::OperatorP,
                    'r' => State::OrR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('o');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::OperatorP => {
                state = match current {
                    'e' => State::OperatorE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("op");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::OperatorE => {
                state = match current {
                    'r' => State::OperatorR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("ope");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::OperatorR => {
                state = match current {
                    'a' => State::OperatorA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("oper");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::OperatorA => {
                state = match current {
                    't' => State::OperatorT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("opera");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::OperatorT => {
                state = match current {
                    'o' => State::OperatorO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("operat");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::OperatorO => {
                state = match current {
                    'r' => State::OperatorR2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("o");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::OperatorR2 => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "operator".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("operator");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `or`
            State::OrR => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "or".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    '_' => Or_eq_,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("or");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `or_eq`
            State::Or_eq_ => {
                state = match current {
                    'e' => State::Or_eqE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("or_");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::Or_eqE => {
                state = match current {
                    'q' => State::Or_eqQ,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("or_e");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::Or_eqQ => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "or_eq".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("or_eq");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `private`
            State::Character('p') => {
                state = match current {
                    'r' => State::PrR,
                    'u' => State::PublicU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('p');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PrR => {
                state = match current {
                    'i' => State::PrivateI,
                    'o' => State::ProtectedO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("pr");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PrivateI => {
                state = match current {
                    'v' => State::PrivateV,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("pri");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PrivateV => {
                state = match current {
                    'a' => State::PrivateA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("priv");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PrivateA => {
                state = match current {
                    't' => State::PrivateT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("priva");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PrivateT => {
                state = match current {
                    'e' => State::PrivateE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("privat");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PrivateE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "private".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("private");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `protected`
            State::ProtectedO => {
                state = match current {
                    't' => State::ProtectedT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("pro");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ProtectedT => {
                state = match current {
                    'e' => State::ProtectedE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("prot");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ProtectedE => {
                state = match current {
                    'c' => State::ProtectedC,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("prote");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ProtectedC => {
                state = match current {
                    't' => State::ProtectedT2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("protec");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ProtectedT2 => {
                state = match current {
                    'e' => State::ProtectedE2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("protect");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ProtectedE2 => {
                state = match current {
                    'd' => State::ProtectedD,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("protecte");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ProtectedD => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "protected".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("protected");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `public`
            State::PublicU => {
                state = match current {
                    'b' => State::PublicB,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("pu");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PublicB => {
                state = match current {
                    'l' => State::PublicL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("pub");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PublicL => {
                state = match current {
                    'i' => State::PublicI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("publ");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PublicI => {
                state = match current {
                    'c' => State::PublicC,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("publi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::PublicC => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "public".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("public");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `return`
            State::Character('r') => {
                state = match current {
                    'e' => State::ReturnE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('r');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ReturnE => {
                state = match current {
                    't' => State::ReturnT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("re");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ReturnT => {
                state = match current {
                    'u' => State::ReturnU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("ret");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ReturnU => {
                state = match current {
                    'r' => State::ReturnR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("retu");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ReturnR => {
                state = match current {
                    'e' => State::ReturnE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("r");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ReturnN => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "return".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("return");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `short`
            State::Character('s') => {
                state = match current {
                    'h' => State::ShortH,
                    'i' => State::SiI,
                    't' => State::StT,
                    'w' => State::SwitchW,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('s');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ShortH => {
                state = match current {
                    'o' => State::ShortO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sh");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ShortO => {
                state = match current {
                    'r' => State::ShortR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sho");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ShortR => {
                state = match current {
                    't' => State::ShortT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("shor");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ShortT => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "short".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("short");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `signed`
            State::SiI => {
                state = match current {
                    'g' => State::SignedG,
                    'z' => State::SizeofZ,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("si");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SignedG => {
                state = match current {
                    'n' => State::SignedN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sig");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SignedN => {
                state = match current {
                    'e' => State::SignedE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sign");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SignedE => {
                state = match current {
                    'd' => State::SignedD,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("signe");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SignedD => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "signed".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("signed");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `sizeof`
            State::SizeofZ => {
                state = match current {
                    'e' => State::SizeofE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("siz");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SizeofE => {
                state = match current {
                    'o' => State::SizeofO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("size");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SizeofO => {
                state = match current {
                    'f' => State::SizeofF,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sizeo");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SizeofF => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "sizeof".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sizeof");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `static`
            State::StT => {
                state = match current {
                    'a' => State::StaticA,
                    't' => State::StructR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sta");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }

            State::StaticA => {
                state = match current {
                    't' => State::StaticT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sta");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::StaticT => {
                state = match current {
                    'a' => State::StaticA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("st");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::StaticI => {
                state = match current {
                    'c' => State::StaticC,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("stati");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::StaticC => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "static".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("static");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `struct`
            State::StructR => {
                state = match current {
                    'u' => State::StructU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("str");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::StructU => {
                state = match current {
                    'c' => State::StructC,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("stru");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::StructC => {
                state = match current {
                    't' => State::StructT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("struc");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::StructT => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "struct".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("struct");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `switch`
            State::SwitchW => {
                state = match current {
                    'i' => State::SwitchI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("sw");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SwitchI => {
                state = match current {
                    't' => State::SwitchT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("swi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SwitchT => {
                state = match current {
                    'c' => State::SwitchC,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("swit");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SwitchC => {
                state = match current {
                    'h' => State::SwitchH,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("switc");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::SwitchH => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "switch".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("switch");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `template`
            State::Character('t') => {
                state = match current {
                    'e' => State::TemplateE,
                    'r' => State::TrR,
                    'h' => State::ThH,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('t');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TemplateE => {
                state = match current {
                    'm' => State::TemplateM,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("te");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TemplateM => {
                state = match current {
                    'p' => State::TemplateP,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("tem");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TemplateP => {
                state = match current {
                    'l' => State::TemplateL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("temp");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TemplateL => {
                state = match current {
                    'a' => State::TemplateA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("templ");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TemplateA => {
                state = match current {
                    't' => State::TemplateT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("templa");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TemplateT => {
                state = match current {
                    'e' => State::TemplateE2,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("t");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TemplateE2 => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "template".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("template");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `this`
            State::ThH => {
                state = match current {
                    'i' => State::ThisI,
                    'r' => State::ThrowR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("th");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ThisI => {
                state = match current {
                    's' => State::ThisS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("thi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ThisS => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "this".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("this");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `throw`
            State::ThrowR => {
                state = match current {
                    'o' => State::ThrowO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("thr");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ThrowO => {
                state = match current {
                    'w' => State::ThrowW,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("thro");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::ThrowW => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "throw".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("throw");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `true`
            State::TrR => {
                state = match current {
                    'y' => State::TryY,
                    'u' => State::TrueU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("tru");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TrueU => {
                state = match current {
                    'e' => State::TrueE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("tru");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::TrueE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "true".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("true");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `try`
            State::TryY => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "try".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("try");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `union`
            State::Character('u') => {
                state = match current {
                    'n' => State::UnN,
                    's' => State::UsingS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('u');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnN => {
                state = match current {
                    'i' => State::UnionI,
                    's' => State::UnsignedS,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("un");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnionI => {
                state = match current {
                    'o' => State::UnionO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("uni");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnionO => {
                state = match current {
                    'n' => State::UnionN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("unio");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnionN => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "union".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("union");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `unsigned`
            State::UnsignedS => {
                state = match current {
                    'i' => State::UnsignedI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("uns");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnsignedI => {
                state = match current {
                    'g' => State::UnsignedG,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("unsi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnsignedG => {
                state = match current {
                    'n' => State::UnsignedN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("unsig");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnsignedN => {
                state = match current {
                    'e' => State::UnsignedE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("unsign");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnsignedE => {
                state = match current {
                    'd' => State::UnsignedD,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("unsigne");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UnsignedD => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "unsigned".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("unsigned");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `using`
            State::UsingS => {
                state = match current {
                    'i' => State::UsingI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("us");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UsingI => {
                state = match current {
                    'n' => State::UsingN,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("usi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UsingN => {
                state = match current {
                    'g' => State::UsingG,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("usin");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::UsingG => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "using".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("using");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `virtual`
            State::Character('v') => {
                state = match current {
                    'i' => State::VirtualI,
                    'o' => State::VoidO,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('v');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VirtualI => {
                state = match current {
                    'r' => State::VirtualR,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("vi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VirtualR => {
                state = match current {
                    't' => State::VirtualT,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("vir");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VirtualT => {
                state = match current {
                    'u' => State::VirtualU,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("virt");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VirtualU => {
                state = match current {
                    'a' => State::VirtualA,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("virtu");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VirtualA => {
                state = match current {
                    'l' => State::VirtualL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("virtua");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VirtualL => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "virtual".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("virtual");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `void`
            State::VoidO => {
                state = match current {
                    'i' => State::VoidI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("vo");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VoidI => {
                state = match current {
                    'd' => State::VoidD,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("voi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::VoidD => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "void".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("void");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
// `while`
            State::Character('w') => {
                state = match current {
                    'h' => State::WhileH,
                    c if c.is_alphanumeric() => {
                        temporary_text.push('w');
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::WhileH => {
                state = match current {
                    'i' => State::WhileI,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("wh");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::WhileI => {
                state = match current {
                    'l' => State::WhileL,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("whi");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::WhileL => {
                state = match current {
                    'e' => State::WhileE,
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("whil");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::WhileE => {
                state = match current {
                    ' ' | '\t' => {
                        tokens.push(Token {
                            token_type: TokenType::Keyword,
                            token: "while".to_owned()
                        });
                        State::WhitespaceIdentifier
                    }
                    c if c.is_alphanumeric() => {
                        temporary_text.push_str("while");
                        temporary_text.push(c);
                        State::Letter(c)
                    },
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::Letter(c) => {
                state = match current {
                    ' ' | '\t' | ';' => {
                        temporary_text.push(c);
                        tokens.push(Token {
                            token_type: TokenType::Identifier,
                            token: temporary_text.to_owned()
                        });
                        temporary_text.clear();
                        State::WhitespaceIdentifier

                    }
                    c if c.is_alphanumeric() || c == '_' => {
                        temporary_text.push(c);
                        State::Letter(c)
                    }
                    _ => return Err(IncorrectIdentifier)
                };
                current_idx += 1;
            }
            State::Number(char) => {
                state = match current {
                    ' ' | '\t' | ';' => {
                        temporary_text.push(current);
                        tokens.push(Token {
                            token_type: TokenType::Identifier,
                            token: temporary_text.to_owned()
                        });
                        temporary_text.clear();
                        State::Whitespace
                    }
                    _ => return Err(Error::VariableNotDefined)
                }
            }
            _ => todo!()

        }
    }
    Ok(tokens)
}