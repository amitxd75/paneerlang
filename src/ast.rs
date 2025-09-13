//! Abstract Syntax Tree (AST) definitions for PaneerLang
//!
//! This module contains all the data structures that represent the parsed
//! structure of PaneerLang programs, including types, statements, and expressions.

/// Represents the type system of PaneerLang
/// Supports primitive types and generic arrays
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// 64-bit signed integer
    Int,
    /// 64-bit floating point number
    Float,
    /// UTF-8 string
    String,
    /// Boolean value (true/false)
    Bool,
    /// Homogeneous array of a specific type
    Array(Box<Type>),
}

/// Root node of the AST representing a complete PaneerLang program
#[derive(Debug, Clone)]
pub struct Program {
    /// List of top-level statements in the program
    pub statements: Vec<Statement>,
}

/// Represents all possible statement types in PaneerLang
#[derive(Debug, Clone)]
pub enum Statement {
    /// Variable declaration: `ye name: type = value;`
    VarDecl {
        name: String,
        type_annotation: Type,
        initializer: Expression,
    },
    /// Function declaration: `func name(params) return_type { body }`
    FuncDecl {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    /// Expression statement: any expression followed by semicolon
    ExprStmt { expression: Expression },
    /// If statement: `agar condition { then_branch } varna { else_branch }`
    IfStmt {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    /// Return statement: `return value;` or `wapas kar value;`
    ReturnStmt { value: Option<Expression> },
    /// While loop: `jabtak condition { body }`
    WhileStmt {
        condition: Expression,
        body: Vec<Statement>,
    },
    /// For loop: `har variable mein iterable { body }`
    ForStmt {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Variable {
        name: String,
    },
    Literal {
        value: LiteralValue,
    },
    MethodCall {
        object: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
    },
    ArrayLiteral {
        elements: Vec<Expression>,
    },
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<LiteralValue>),
}

impl LiteralValue {
    /// Returns the PaneerLang type corresponding to this literal value
    pub fn get_type(&self) -> Type {
        match self {
            LiteralValue::Int(_) => Type::Int,
            LiteralValue::Float(_) => Type::Float,
            LiteralValue::String(_) => Type::String,
            LiteralValue::Bool(_) => Type::Bool,
            LiteralValue::Array(arr) => {
                if arr.is_empty() {
                    Type::Array(Box::new(Type::Int)) // Default to int array
                } else {
                    Type::Array(Box::new(arr[0].get_type()))
                }
            }
        }
    }

    /// Determines if this value is considered "truthy" in boolean contexts
    ///
    /// # Returns
    /// * `true` for non-zero numbers, non-empty strings/arrays, and boolean true
    /// * `false` for zero, empty strings/arrays, and boolean false
    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::Bool(b) => *b,
            LiteralValue::Int(i) => *i != 0,
            LiteralValue::Float(f) => *f != 0.0,
            LiteralValue::String(s) => !s.is_empty(),
            LiteralValue::Array(arr) => !arr.is_empty(),
        }
    }
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Int(i) => write!(f, "{}", i),
            LiteralValue::Float(fl) => write!(f, "{}", fl),
            LiteralValue::String(s) => write!(f, "{}", s),
            LiteralValue::Bool(b) => write!(f, "{}", b),
            LiteralValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Array(inner) => write!(f, "array<{}>", inner),
        }
    }
}
