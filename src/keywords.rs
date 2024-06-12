#[derive(Debug)]
pub enum Error {
    IncorrectIdentifier(Location),
    IncorrectKeyword(Location),
    IncorrectOperator(Location),
    IncorrectConstant(Location),
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::IncorrectIdentifier(l) => {
                format!("Identifier error at {}:{} ({})", l.line, l.column, l.char)
            }
            Self::IncorrectKeyword(l) => {
                format!("Keyword error at {}:{} ({})", l.line, l.column, l.char)
            }
            Self::IncorrectOperator(l) => {
                format!("Operator error at {}:{} ({})", l.line, l.column, l.char)
            }
            Self::IncorrectConstant(l) => {
                format!("Constant error at {}:{} ({})", l.line, l.column, l.char)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    ConstValue,
    StringLiteral,
    Separator,
    // None,
}

#[derive(Debug)]
enum State {
    // Intermediate
    /// \t, spaces, \n
    Whitespace,
    Base,
    /// '(',')','[',']', '{', '}', ';'
    Separator(char),
    /// Generic state for letters
    Letter(char),
    /// State for identifier characters
    Identifier(char),
    /// Fallback state for characters
    Character(char),
    /// Generic state for numbers
    Number(char),
    /// _, used for variable name starts
    Underscore,
    /// Anything that starts with a # is a preprocessor command
    Preprocessor,

    Operator(char),
    KeywordEnd,
    StringLiteral(char),
    // Operators:
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// <<
    Shl,
    /// >>
    Shr,
    /// &&
    And,
    /// ||
    Or,
    /// ^
    BitXor,
    /// &
    BitAnd,
    /// |
    BitOr,
    /// ++
    Incr,
    /// --
    Decr,
    // Comparison operators:
    /// \<
    LT,
    /// \>
    GT,
    /// <=
    LE,
    /// >=
    GE,
    /// ==
    Eq,
    /// !=
    NEq,
    /// !
    Neg,
    // Assign operators:
    /// =
    Assign,
    /// +=
    AddAssign,
    /// -=
    SubAssign,
    /// *=
    MulAssign,
    /// /=
    DivAssign,
    /// %=
    ModAssign,
    /// <\<\=
    ShlAssign,
    /// \>\>\=
    ShrAssign,
    /// &=
    BitAndAssign,
    /// |=
    BitOrAssign,
    // Special operators:
    /// ,
    Comma,
    /// ->
    Arrow,

    // Keywords
    /// asm
    AsmS,
    /// auto
    AutoU,
    AutoT,
    /// bool
    BoolO,
    BoolO2,
    /// break
    BreakR,
    BreakE,
    BreakA,
    /// case | catch
    CaA,
    /// case
    CaseS,
    /// catch
    CatchT,
    CatchC,
    /// char
    CharH,
    CharA,
    /// class
    ClassL,
    ClassA,
    ClassS,
    /// compl | concept | const
    CoO,
    /// compl
    ComplM,
    ComplP,
    /// concept | const
    ConN,
    /// concept
    ConceptC,
    ConceptE,
    ConceptP,
    /// const
    ConstS,
    /// default | delete
    DeE,
    DefaultF,
    DefaultA,
    DefaultU,
    DefaultL,
    /// Delete
    DeleteL,
    DeleteE,
    DeleteT,
    /// do | double
    DoO,
    /// double
    DoubleU,
    DoubleB,
    DoubleL,
    /// else
    ElseL,
    ElseS,
    /// enum
    EnumN,
    EnumU,
    /// export | extern
    ExX,
    /// export
    ExportP,
    ExportO,
    ExportR,
    /// extern
    ExternT,
    ExternE,
    ExternR,
    /// false
    FalseA,
    FalseL,
    FalseS,
    /// float
    FloatL,
    FloatO,
    FloatA,
    /// for
    ForO,
    /// friend
    FriendR,
    FriendI,
    FriendE,
    FriendN,
    /// goto
    GotoO,
    GotoT,
    /// if
    IfF,
    /// int | inline
    InN,
    /// inline
    InlineL,
    InlineI,
    InlineN2,
    /// int
    /// long
    LongO,
    LongN,
    /// mutable
    MutableU,
    MutableT,
    MutableA,
    MutableB,
    MutableL,
    /// namespace
    NamespaceA,
    NamespaceM,
    NamespaceE,
    NamespaceS,
    NamespaceP,
    NamespaceA2,
    NamespaceC,
    NewE,
    /// nullptr
    NullptrU,
    NullptrL,
    NullptrL2,
    NullptrP,
    NullptrT,
    /// operator
    OperatorP,
    OperatorE,
    OperatorR,
    OperatorA,
    OperatorT,
    OperatorO,
    /// private | protected
    PrR,
    /// private
    PrivateI,
    PrivateV,
    PrivateA,
    PrivateT,
    /// protected
    ProtectedO,
    ProtectedT,
    ProtectedE,
    ProtectedC,
    ProtectedT2,
    ProtectedE2,
    /// public
    PublicU,
    PublicB,
    PublicL,
    PublicI,
    /// return
    ReturnE,
    ReturnT,
    ReturnU,
    ReturnR,
    /// short
    ShortH,
    ShortO,
    ShortR,
    /// signed || sizeof
    SiI,
    /// signed
    SignedG,
    SignedN,
    SignedE,
    /// sizeof
    SizeofZ,
    SizeofE,
    SizeofO,
    /// static | struct
    StT,
    /// static
    StaticA,
    StaticT,
    StaticI,
    /// struct
    StructR,
    StructU,
    StructC,
    /// switch
    SwitchW,
    SwitchI,
    SwitchT,
    SwitchC,
    /// template
    TemplateE,
    TemplateM,
    TemplateP,
    TemplateL,
    TemplateA,
    TemplateT,
    /// this | throw
    ThH,
    /// this
    ThisI,
    /// throw
    ThrowR,
    ThrowO,
    /// true | try
    TrR,
    /// true
    TrueU,
    /// try
    /// union | unsigned
    UnN,
    /// union
    UnionI,
    UnionO,
    /// unsigned
    UnsignedS,
    UnsignedI,
    UnsignedG,
    UnsignedN,
    UnsignedE,
    /// using
    UsingS,
    UsingI,
    UsingN,
    /// virtual
    VirtualI,
    VirtualR,
    VirtualT,
    VirtualU,
    VirtualA,
    /// void
    VoidO,
    VoidI,
    /// while
    WhileH,
    WhileI,
    WhileL,
    NumberAfterExponent(char),
    NumberAfterDot(char),
    NumberAfterExponentWithSign(char),
}
fn is_separator(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' | ':' => true,
        _ => false,
    }
}
fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false,
    }
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
    let mut location: Location = Location {
        line: 0,
        column: 0,
        char: ' ',
    };
    let mut state: State = State::Whitespace;
    let mut token_type: TokenType = TokenType::Identifier;
    let mut buff: String = String::new();
    let mut is_writable: bool = false;
    let mut current_idx: usize = 0;
    let mut current: char;
    while current_idx < text.chars().count() {
        current = text.as_str().as_bytes()[current_idx] as char;
        location.char = current;
        location.column += 1;
        // if current == '\n' {
        //     location.column = 0;
        //     location.line += 1;
        // }
        match state {
            State::Whitespace => {
                if current == '\n' {
                    location.column = 0;
                    location.line += 1;
                };
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    '+' => {
                        buff.push(current);
                        State::Add
                    }
                    '-' => {
                        buff.push(current);
                        State::Sub
                    }
                    '*' => {
                        buff.push(current);
                        State::Mul
                    }
                    '/' => {
                        buff.push(current);
                        State::Div
                    }
                    '%' => {
                        buff.push(current);
                        State::Mod
                    }
                    '<' => {
                        buff.push(current);
                        State::LT
                    }
                    '>' => {
                        buff.push(current);
                        State::GT
                    }
                    '=' => {
                        buff.push(current);
                        State::Assign
                    }
                    '!' => {
                        buff.push(current);
                        State::Neg
                    }
                    '&' => {
                        buff.push(current);
                        State::BitAnd
                    }
                    '|' => {
                        buff.push(current);
                        State::BitOr
                    }
                    '^' => {
                        buff.push(current);
                        State::BitXor
                    }
                    ',' => {
                        buff.push(current);
                        is_writable = true;
                        State::Separator(current)
                    }
                    '_' => {
                        buff.push(current);
                        State::Underscore
                    }
                    c if c.is_alphabetic() => {
                        buff.push(current);
                        token_type = TokenType::Identifier;
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        buff.push(current);
                        token_type = TokenType::ConstValue;
                        State::Number(c)
                    }
                    c if is_separator(c) => {
                        buff.push(current);
                        is_writable = true;
                        State::Separator(current)
                    }
                    '#' => {
                        buff.push(current);
                        State::Preprocessor
                    }
                    _ => State::Character(current),
                };
            }
            State::Preprocessor => match current {
                c if c.is_alphanumeric() => {
                    state = State::Preprocessor;
                    buff.push(c);
                }
                c if is_whitespace(c) => {
                    is_writable = true;
                    current_idx -= 1;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Underscore => match current {
                c if c.is_alphanumeric() => {
                    buff.push(c);
                    state = State::Identifier(c);
                }
                c if is_whitespace(c) => {
                    is_writable = true;
                    state = State::Whitespace
                }
                _ => return Err(Error::IncorrectIdentifier(location)),
            },
            State::Add => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::AddAssign
                }
                '+' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::Incr
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    current_idx -= 1;
                    is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::Sub => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::SubAssign
                }
                '-' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::Decr
                }
                '>' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::Arrow
                }
                c if c.is_numeric() => {
                    buff.push(current);
                    state = State::Number(c);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    current_idx -= 1;
                    is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::Mul => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::MulAssign
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    current_idx -= 1;
                    is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::Div => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::DivAssign
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    current_idx -= 1;
                    is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::Mod => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::ModAssign
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    current_idx -= 1;
                    is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::Shl => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::ShlAssign
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    // current_idx -= 1;
                    // is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::Shr => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::ShrAssign
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    current_idx -= 1;
                    is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::And => match current {
                '&' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::BitAnd;
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    current_idx -= 1;
                    is_writable = true;
                    // return Err(Error::IncorrectOperator(location));
                }
            },
            State::Or => match current {
                '|' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::BitOr;
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::BitXor => match current {
                c if is_whitespace(c) || is_separator(c) => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::BitAnd => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::BitAndAssign
                }
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::BitOr => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::BitOrAssign
                }
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::Incr => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::Decr => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::LT => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::LE
                }
                '<' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::Shl
                }
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::GT => match current {
                '=' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::GE
                }
                '>' => {
                    buff.push(current);
                    is_writable = true;
                    state = State::Shr
                }
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::LE => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::GE => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::Eq => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::NEq => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::Neg => match current {
                '=' => {
                    buff.push(current);
                    state = State::NEq
                }
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::Assign => {
                match current {
                    '=' => {
                        buff.push(current);
                        state = State::Eq
                    }
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        current_idx -= 1;
                        is_writable = true;
                    } // return Err(Error::IncorrectOperator(location)),
                };
            }
            State::AddAssign => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::SubAssign => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::MulAssign => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::DivAssign => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::ModAssign => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::ShlAssign => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::ShrAssign => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::BitAndAssign => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::BitOrAssign => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectOperator(location)),
            },
            State::Comma => {
                match current {
                    ' ' | '\n' | '\t' => state = State::Whitespace,
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::Arrow => {
                match current {
                    c if is_whitespace(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::Letter('a') => {
                match current {
                    's' => {
                        buff.push(current);
                        state = State::AsmS
                    }
                    'u' => {
                        buff.push(current);
                        state = State::AutoU;
                    }
                    c if c.is_alphanumeric() || c == '_' => {
                        buff.push(current);
                        state = State::Identifier(c)
                    }
                    c if is_whitespace(c) || is_separator(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::AsmS => {
                match current {
                    'm' => {
                        buff.push(current);
                        state = State::KeywordEnd
                    }
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        state = State::Identifier(c)
                    }
                    c if is_whitespace(c) || is_separator(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::AutoU => {
                match current {
                    't' => {
                        buff.push(current);
                        state = State::AutoT;
                    }
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        state = State::Identifier(c);
                    }
                    c if is_whitespace(c) || is_separator(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::AutoT => {
                match current {
                    'o' => {
                        buff.push(current);
                        state = State::KeywordEnd
                    }
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' => state = State::Whitespace,

                    _ => return Err(Error::IncorrectOperator(location)),
                };
            }
            State::Letter('b') => match current {
                'o' => {
                    buff.push(current);
                    state = State::BoolO
                }
                'r' => {
                    buff.push(current);
                    state = State::BreakR
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::BoolO => match current {
                'o' => {
                    state = State::BoolO2;
                }
                c if c.is_alphanumeric() || c == '_' => {
                    state = State::Identifier(c);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::BoolO2 => {
                match current {
                    'l' => {
                        buff.push(current);
                        state = State::KeywordEnd
                    }
                    c if c.is_alphanumeric() => state = State::Letter(c),
                    c if is_whitespace(c) || is_separator(c) => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => return Err(Error::IncorrectKeyword(location)),
                };
            }
            State::BreakR => match current {
                'e' => {
                    buff.push(current);
                    state = State::BreakE
                }
                c if c.is_alphanumeric() => state = State::Identifier(c),
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::BreakE => match current {
                'a' => {
                    buff.push(current);
                    state = State::BreakA
                }
                c if c.is_alphanumeric() => state = State::Letter(c),
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::BreakA => match current {
                'k' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() => state = State::Letter(c),
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('c') => match current {
                'a' => {
                    buff.push(current);
                    state = State::CaA
                }
                'h' => {
                    buff.push(current);
                    state = State::CharH
                }
                'o' => {
                    buff.push(current);
                    state = State::CoO
                }
                c if c.is_alphanumeric() => state = State::Letter(current),
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::CaA => match current {
                's' => {
                    buff.push(current);
                    state = State::CaseS
                }
                't' => {
                    buff.push(current);
                    state = State::CatchT
                }
                c if c.is_alphanumeric() => state = State::Identifier(c),
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::CaseS => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() => state = State::Identifier(c),
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::CatchT => match current {
                'c' => {
                    buff.push(current);
                    state = State::CatchC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::CatchC => match current {
                'h' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::CharH => match current {
                'a' => {
                    buff.push(current);
                    state = State::CharA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::CharA => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ClassL => match current {
                'a' => {
                    buff.push(current);
                    state = State::ClassA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ClassA => match current {
                's' => {
                    buff.push(current);
                    state = State::ClassS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ClassS => match current {
                's' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::CoO => match current {
                'm' => {
                    buff.push(current);
                    state = State::ComplM
                }
                'n' => {
                    buff.push(current);
                    state = State::ConN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ComplM => match current {
                'p' => {
                    buff.push(current);
                    state = State::ComplP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ComplP => match current {
                'l' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ConN => match current {
                'c' => {
                    buff.push(current);
                    state = State::ConceptC
                }
                's' => {
                    buff.push(current);
                    state = State::ConstS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ConceptC => match current {
                'e' => {
                    buff.push(current);
                    state = State::ConceptE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ConceptE => match current {
                'p' => {
                    buff.push(current);
                    state = State::ConceptP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ConceptP => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ConstS => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('d') => match current {
                'e' => {
                    buff.push(current);
                    state = State::DeE
                }
                'o' => {
                    buff.push(current);
                    state = State::DoO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DeE => match current {
                'f' => {
                    buff.push(current);
                    state = State::DefaultF
                }
                'l' => {
                    buff.push(current);
                    state = State::DeleteE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DefaultF => match current {
                'a' => {
                    buff.push(current);
                    state = State::DefaultF
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DefaultA => match current {
                'u' => {
                    buff.push(current);
                    state = State::DefaultU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DefaultU => match current {
                'l' => {
                    buff.push(current);
                    state = State::DefaultL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DefaultL => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DeleteL => match current {
                'e' => {
                    buff.push(current);
                    state = State::DeleteE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DeleteE => match current {
                't' => {
                    buff.push(current);
                    state = State::DeleteT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DeleteT => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DoO => match current {
                'u' => {
                    buff.push(current);
                    state = State::DoubleU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DoubleU => match current {
                'b' => {
                    buff.push(current);
                    state = State::DoubleB
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DoubleB => match current {
                'l' => {
                    buff.push(current);
                    state = State::DoubleL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::DoubleL => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('e') => match current {
                'l' => {
                    buff.push(current);
                    state = State::ElseL
                }
                'n' => {
                    buff.push(current);
                    state = State::EnumN
                }
                'x' => {
                    buff.push(current);
                    state = State::ExX
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ElseL => match current {
                's' => {
                    buff.push(current);
                    state = State::ElseS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ElseS => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::EnumN => match current {
                'u' => {
                    buff.push(current);
                    state = State::EnumU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::EnumU => match current {
                'm' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::EnumM => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExX => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExportP => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExportO => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExportR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExportT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExternT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExternE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExternR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            // State::ExternN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::Letter('f') => match current {
                'a' => {
                    buff.push(current);
                    state = State::FalseA
                }
                'l' => {
                    buff.push(current);
                    state = State::FloatL
                }
                'o' => {
                    buff.push(current);
                    state = State::ForO
                }
                'r' => {
                    buff.push(current);
                    state = State::FriendR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FalseA => match current {
                'l' => {
                    buff.push(current);
                    state = State::FalseL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FalseL => match current {
                's' => {
                    buff.push(current);
                    state = State::FalseS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FalseS => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FloatL => match current {
                'o' => {
                    buff.push(current);
                    state = State::FloatO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FloatO => match current {
                'a' => {
                    buff.push(current);
                    state = State::FloatA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FloatA => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ForO => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FriendR => match current {
                'i' => {
                    buff.push(current);
                    state = State::FriendI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FriendI => match current {
                'e' => {
                    buff.push(current);
                    state = State::FriendE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FriendE => match current {
                'n' => {
                    buff.push(current);
                    state = State::FriendN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::FriendN => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('g') => match current {
                'o' => {
                    buff.push(current);
                    state = State::GotoO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::GotoO => match current {
                't' => {
                    buff.push(current);
                    state = State::GotoT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::GotoT => match current {
                'o' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('i') => match current {
                'f' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                'n' => {
                    buff.push(current);
                    state = State::InN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::IfF => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::InN => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                'l' => {
                    buff.push(current);
                    state = State::InlineL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::InlineL => match current {
                'i' => {
                    buff.push(current);
                    state = State::InlineI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::InlineI => match current {
                'n' => {
                    buff.push(current);
                    state = State::InlineN2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::InlineN2 => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('l') => match current {
                'f' => {
                    buff.push(current);
                    state = State::LongO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::LongO => match current {
                'n' => {
                    buff.push(current);
                    state = State::LongN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::LongN => match current {
                'g' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('m') => match current {
                'u' => {
                    buff.push(current);
                    state = State::MutableU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::MutableU => match current {
                't' => {
                    buff.push(current);
                    state = State::MutableT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::MutableT => match current {
                'a' => {
                    buff.push(current);
                    state = State::MutableA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::MutableA => match current {
                'b' => {
                    buff.push(current);
                    state = State::MutableB
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::MutableB => match current {
                'l' => {
                    buff.push(current);
                    state = State::MutableL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::MutableL => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('n') => match current {
                'a' => {
                    buff.push(current);
                    state = State::NamespaceA
                }
                'e' => {
                    buff.push(current);
                    state = State::NewE
                }
                'u' => {
                    buff.push(current);
                    state = State::NullptrU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NamespaceA => match current {
                'm' => {
                    buff.push(current);
                    state = State::NamespaceM
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NamespaceM => match current {
                'e' => {
                    buff.push(current);
                    state = State::NamespaceE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NamespaceE => match current {
                's' => {
                    buff.push(current);
                    state = State::NamespaceS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NamespaceS => match current {
                'p' => {
                    buff.push(current);
                    state = State::NamespaceP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NamespaceP => match current {
                'a' => {
                    buff.push(current);
                    state = State::NamespaceA2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NamespaceA2 => match current {
                'c' => {
                    buff.push(current);
                    state = State::NamespaceC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NamespaceC => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NewE => match current {
                'w' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NullptrU => match current {
                'l' => {
                    buff.push(current);
                    state = State::NullptrL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NullptrL => match current {
                'l' => {
                    buff.push(current);
                    state = State::NullptrL2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NullptrL2 => match current {
                'p' => {
                    buff.push(current);
                    state = State::NullptrP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NullptrP => match current {
                't' => {
                    buff.push(current);
                    state = State::NullptrT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NullptrT => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::OperatorP => match current {
                'e' => {
                    buff.push(current);
                    state = State::OperatorE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::OperatorE => match current {
                'r' => {
                    buff.push(current);
                    state = State::OperatorR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::OperatorR => match current {
                'a' => {
                    buff.push(current);
                    state = State::OperatorA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::OperatorA => match current {
                't' => {
                    buff.push(current);
                    state = State::OperatorT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::OperatorT => match current {
                'o' => {
                    buff.push(current);
                    state = State::OperatorO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::OperatorO => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('p') => match current {
                'r' => {
                    buff.push(current);
                    state = State::PrR
                }
                'u' => {
                    buff.push(current);
                    state = State::PublicU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PrR => match current {
                'o' => {
                    buff.push(current);
                    state = State::ProtectedO
                }
                'i' => {
                    buff.push(current);
                    state = State::PrivateI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PrivateI => match current {
                'v' => {
                    buff.push(current);
                    state = State::PrivateV
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PrivateV => match current {
                'a' => {
                    buff.push(current);
                    state = State::PrivateA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PrivateA => match current {
                't' => {
                    buff.push(current);
                    state = State::PrivateT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PrivateT => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ProtectedO => match current {
                't' => {
                    buff.push(current);
                    state = State::ProtectedT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ProtectedT => match current {
                'e' => {
                    buff.push(current);
                    state = State::ProtectedC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ProtectedE => match current {
                'c' => {
                    buff.push(current);
                    state = State::ProtectedC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ProtectedC => match current {
                't' => {
                    buff.push(current);
                    state = State::ProtectedT2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ProtectedT2 => match current {
                'e' => {
                    buff.push(current);
                    state = State::ProtectedE2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ProtectedE2 => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PublicU => match current {
                'b' => {
                    buff.push(current);
                    state = State::PublicB
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PublicB => match current {
                'l' => {
                    buff.push(current);
                    state = State::PublicL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PublicL => match current {
                'i' => {
                    buff.push(current);
                    state = State::PublicI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::PublicI => match current {
                'c' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('r') => match current {
                'e' => {
                    buff.push(current);
                    state = State::ReturnE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ReturnE => match current {
                't' => {
                    buff.push(current);
                    state = State::ReturnT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ReturnT => match current {
                'u' => {
                    buff.push(current);
                    state = State::ReturnU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ReturnU => match current {
                'r' => {
                    buff.push(current);
                    state = State::ReturnR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ReturnR => match current {
                'n' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('s') => match current {
                'h' => {
                    buff.push(current);
                    state = State::ShortH
                }
                'i' => {
                    buff.push(current);
                    state = State::SiI
                }
                't' => {
                    buff.push(current);
                    state = State::StT
                }
                'w' => {
                    buff.push(current);
                    state = State::SwitchW
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ShortH => match current {
                'o' => {
                    buff.push(current);
                    state = State::ShortO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ShortO => match current {
                'r' => {
                    buff.push(current);
                    state = State::ShortR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ShortR => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SiI => match current {
                'g' => {
                    buff.push(current);
                    state = State::SignedG
                }
                'z' => {
                    buff.push(current);
                    state = State::SizeofZ
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SignedG => match current {
                'n' => {
                    buff.push(current);
                    state = State::SignedN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SignedN => match current {
                'e' => {
                    buff.push(current);
                    state = State::SignedE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SignedE => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SizeofZ => match current {
                'e' => {
                    buff.push(current);
                    state = State::SizeofE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SizeofE => match current {
                'o' => {
                    buff.push(current);
                    state = State::SizeofO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SizeofO => match current {
                'f' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StT => match current {
                'a' => {
                    buff.push(current);
                    state = State::StaticA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StaticA => match current {
                't' => {
                    buff.push(current);
                    state = State::StaticT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StaticT => match current {
                'i' => {
                    buff.push(current);
                    state = State::StaticI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StaticI => match current {
                'c' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StructR => match current {
                'u' => {
                    buff.push(current);
                    state = State::StructU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StructU => match current {
                'c' => {
                    buff.push(current);
                    state = State::StructC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StructC => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SwitchW => match current {
                'i' => {
                    buff.push(current);
                    state = State::SwitchI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SwitchI => match current {
                't' => {
                    buff.push(current);
                    state = State::SwitchT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SwitchT => match current {
                'c' => {
                    buff.push(current);
                    state = State::SwitchC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::SwitchC => match current {
                'h' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('t') => match current {
                'e' => {
                    buff.push(current);
                    state = State::TemplateE
                }
                'h' => {
                    buff.push(current);
                    state = State::ThH
                }
                'r' => {
                    buff.push(current);
                    state = State::TrR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::TemplateE => match current {
                'm' => {
                    buff.push(current);
                    state = State::TemplateM
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::TemplateM => match current {
                'p' => {
                    buff.push(current);
                    state = State::TemplateP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::TemplateP => match current {
                'l' => {
                    buff.push(current);
                    state = State::TemplateL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::TemplateL => match current {
                'a' => {
                    buff.push(current);
                    state = State::TemplateA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::TemplateA => match current {
                't' => {
                    buff.push(current);
                    state = State::TemplateT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::TemplateT => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ThH => match current {
                'i' => {
                    buff.push(current);
                    state = State::ThisI
                }
                'r' => {
                    buff.push(current);
                    state = State::ThrowR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ThisI => match current {
                's' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ThrowR => match current {
                'o' => {
                    buff.push(current);
                    state = State::ThrowO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::ThrowO => match current {
                'w' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::ThrowW => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::TrR => match current {
                'y' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                'u' => {
                    buff.push(current);
                    state = State::TrueU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::TrueU => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter('u') => match current {
                'n' => {
                    buff.push(current);
                    state = State::UnN
                }
                's' => {
                    buff.push(current);
                    state = State::UsingS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UnN => match current {
                'i' => {
                    buff.push(current);
                    state = State::UnionI
                }
                's' => {
                    buff.push(current);
                    state = State::UnsignedS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UnionI => match current {
                'o' => {
                    buff.push(current);
                    state = State::UnionO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UnionO => match current {
                'n' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::UnionN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::UnsignedS => match current {
                'i' => {
                    buff.push(current);
                    state = State::UnsignedI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UnsignedI => match current {
                'g' => {
                    buff.push(current);
                    state = State::UnsignedG
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UnsignedG => match current {
                'n' => {
                    buff.push(current);
                    state = State::UnsignedN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UnsignedN => match current {
                'e' => {
                    buff.push(current);
                    state = State::UnsignedE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UnsignedE => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::UnsignedD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::UsingS => match current {
                'i' => {
                    buff.push(current);
                    state = State::UsingI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UsingI => match current {
                'n' => {
                    buff.push(current);
                    state = State::UsingN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::UsingN => match current {
                'g' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::UsingG => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::Letter('v') => match current {
                'i' => {
                    buff.push(current);
                    state = State::VirtualI
                }
                'o' => {
                    buff.push(current);
                    state = State::VoidO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::VirtualI => match current {
                'r' => {
                    buff.push(current);
                    state = State::VirtualR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::VirtualR => match current {
                't' => {
                    buff.push(current);
                    state = State::VirtualT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::VirtualT => match current {
                'u' => {
                    buff.push(current);
                    state = State::VirtualU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::VirtualU => match current {
                'a' => {
                    buff.push(current);
                    state = State::VirtualA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::VirtualA => match current {
                'l' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::VirtualL => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::VoidO => match current {
                'i' => {
                    buff.push(current);
                    state = State::VoidI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::VoidI => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::VoidD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::Letter('w') => match current {
                'h' => {
                    buff.push(current);
                    state = State::WhileH
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::WhileH => match current {
                'h' => {
                    buff.push(current);
                    state = State::WhileI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::WhileI => match current {
                'l' => {
                    buff.push(current);
                    state = State::WhileL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::WhileL => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            // State::WhileE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         c if is_whitespace(c) || is_separator(c) => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => return Err(Error::IncorrectKeyword(location))
            //     }
            // }
            State::KeywordEnd => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },

            State::Separator(s) => match current {
                c if is_whitespace(c) => {
                    current_idx -= 1;
                    is_writable = true;
                    state = State::Whitespace
                }
                c if is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                    state = State::Separator(current)
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Identifier(i) => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectIdentifier(location)),
            },
            State::Character('"') => {
                buff.push(current);
                match current {
                    _ => state = State::StringLiteral(current),
                }
            }
            State::StringLiteral('"') => match current {
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::StringLiteral(c) => match current {
                '"' => state = State::StringLiteral('"'),
                c => {
                    buff.push(current);
                    state = State::StringLiteral(current);
                }
            },
            State::Number('e') | State::Number('E') => match current {
                c if c.is_numeric() || c == '-' || c == '+' || c == '_' => {
                    buff.push(c);
                    state = State::NumberAfterExponent(c);
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::NumberAfterExponent('-') | State::NumberAfterExponent('+') => match current {
                c if c.is_numeric() => {
                    buff.push(c);
                    state = State::NumberAfterExponent(c);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectConstant(location)),
            },
            State::NumberAfterExponent(c) => match current {
                c if c.is_numeric() || c == '-' || c == '+' => {
                    buff.push(c);
                    state = State::NumberAfterExponent(c);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectConstant(location)),
            },
            State::NumberAfterExponentWithSign(c) => match current {
                c if c.is_numeric() => {
                    buff.push(c);
                    state = State::NumberAfterExponentWithSign(c);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectConstant(location)),
            },
            State::NumberAfterDot(c) => match current {
                c if c.is_numeric() => {
                    buff.push(c);
                    state = State::NumberAfterDot(c);
                }
                'e' | 'E' => {
                    buff.push(current);
                    state = State::NumberAfterExponent(current);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectConstant(location)),
            },
            State::Number('.') => match current {
                c if c.is_numeric() => {
                    buff.push(c);
                    state = State::NumberAfterDot(c);
                }
                'e' | 'E' => {
                    buff.push(current);
                    state = State::NumberAfterExponent(current);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectConstant(location)),
            },

            State::Number('-') | State::Number('+') => match current {
                c if c.is_numeric() => {
                    buff.push(c);
                    state = State::Number(c);
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectConstant(location)),
            },
            State::Number(n) => match current {
                c if c.is_numeric() || c == '.' || c == '_' || c == 'e' || c == 'E' => {
                    buff.push(c);
                    state = State::Number(c);
                }
                c if is_whitespace(c) || is_separator(c) => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            State::Letter(l) => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => return Err(Error::IncorrectKeyword(location)),
            },
            _ => {}
        }
        // println!(
        //     "{:?} : {:?}, {} buff: {:?}",
        //     current, state, is_writable, buff
        // );
        current_idx += 1;

        if is_writable {
            let token_type = match state {
                State::Identifier(_)
                | State::Letter(_)
                | State::Underscore
                | State::Preprocessor => TokenType::Identifier,
                State::KeywordEnd | State::DoO => TokenType::Keyword,
                State::Number(_)
                | State::NumberAfterDot(_)
                | State::NumberAfterExponent(_)
                | State::NumberAfterExponentWithSign(_) => TokenType::ConstValue,
                State::Separator(_) => TokenType::Separator,
                State::StringLiteral(_) => TokenType::StringLiteral,
                _ => TokenType::Operator,
            };
            state = State::Whitespace;
            is_writable = false;
            tokens.push(Token {
                token_type,
                token: buff.clone(),
            });
            buff.clear();
        }
    }
    Ok(tokens)
}
