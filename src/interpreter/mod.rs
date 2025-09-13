//! PaneerLang interpreter implementation
//!
//! This module contains the tree-walking interpreter that executes PaneerLang AST nodes.
//! It handles variable scoping, function calls, control flow, and built-in operations.

use crate::ast::*;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Represents a user-defined function in PaneerLang
#[derive(Debug, Clone)]
pub struct Function {
    /// Function parameters with their names and types
    pub params: Vec<(String, Type)>,
    /// Expected return type
    pub return_type: Type,
    /// Function body statements
    pub body: Vec<Statement>,
}

/// Environment for variable and function scoping
///
/// Supports lexical scoping with parent environments for nested scopes
#[derive(Debug, Clone)]
pub struct Environment {
    /// Variables defined in this scope
    variables: HashMap<String, LiteralValue>,
    /// Functions defined in this scope
    functions: HashMap<String, Function>,
    /// Parent environment for lexical scoping
    parent: Option<Box<Environment>>,
}

impl Environment {
    /// Creates a new empty environment with no parent
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    /// Creates a new environment with the given parent for lexical scoping
    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Defines a variable in the current scope
    pub fn define_variable(&mut self, name: String, value: LiteralValue) {
        self.variables.insert(name, value);
    }

    /// Retrieves a variable value, checking parent scopes if not found locally
    pub fn get_variable(&self, name: &str) -> Option<LiteralValue> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_variable(name)
        } else {
            None
        }
    }

    /// Defines a function in the current scope
    pub fn define_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    /// Retrieves a function, checking parent scopes if not found locally
    pub fn get_function(&self, name: &str) -> Option<Function> {
        if let Some(function) = self.functions.get(name) {
            Some(function.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_function(name)
        } else {
            None
        }
    }
}

/// Main interpreter struct that executes PaneerLang programs
pub struct Interpreter {
    /// Current execution environment
    environment: Environment,
}

/// Runtime values that can be returned from statement execution
#[derive(Debug)]
pub enum RuntimeValue {
    /// Normal statement execution (no return)
    Value,
    /// Return statement with a value
    Return(LiteralValue),
}

impl Interpreter {
    /// Creates a new interpreter with an empty global environment
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    /// Interprets a complete PaneerLang program
    ///
    /// # Arguments
    /// * `program` - The parsed AST program to execute
    ///
    /// # Returns
    /// * `Ok(())` - If program executes successfully
    /// * `Err(anyhow::Error)` - If execution fails
    pub fn interpret(&mut self, program: Program) -> Result<()> {
        for statement in program.statements {
            if let RuntimeValue::Return(_) = self.execute_statement(statement)? {
                return Err(anyhow!("Return statement outside of function"));
            }
        }
        Ok(())
    }

    /// Executes a single statement and returns the runtime result
    ///
    /// # Arguments
    /// * `statement` - The statement to execute
    ///
    /// # Returns
    /// * `Ok(RuntimeValue)` - Normal execution or return value
    /// * `Err(anyhow::Error)` - If execution fails
    fn execute_statement(&mut self, statement: Statement) -> Result<RuntimeValue> {
        match statement {
            Statement::VarDecl {
                name,
                type_annotation,
                initializer,
            } => {
                let value = self.evaluate_expression(initializer)?;

                // Type checking
                if value.get_type() != type_annotation {
                    return Err(anyhow!(
                        "Type mismatch: expected {}, got {}",
                        type_annotation,
                        value.get_type()
                    ));
                }

                self.environment.define_variable(name, value);
                Ok(RuntimeValue::Value)
            }

            Statement::FuncDecl {
                name,
                params,
                return_type,
                body,
            } => {
                let function = Function {
                    params,
                    return_type,
                    body,
                };
                self.environment.define_function(name, function);
                Ok(RuntimeValue::Value)
            }

            Statement::ExprStmt { expression } => {
                self.evaluate_expression(expression)?;
                Ok(RuntimeValue::Value)
            }

            Statement::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.evaluate_expression(condition)?;

                if condition_value.is_truthy() {
                    for stmt in then_branch {
                        if let RuntimeValue::Return(val) = self.execute_statement(stmt)? {
                            return Ok(RuntimeValue::Return(val));
                        }
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        if let RuntimeValue::Return(val) = self.execute_statement(stmt)? {
                            return Ok(RuntimeValue::Return(val));
                        }
                    }
                }

                Ok(RuntimeValue::Value)
            }

            Statement::ReturnStmt { value } => {
                let return_value = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    LiteralValue::Int(0) // Default return value
                };
                Ok(RuntimeValue::Return(return_value))
            }

            Statement::WhileStmt { condition, body } => {
                loop {
                    let condition_value = self.evaluate_expression(condition.clone())?;
                    if !condition_value.is_truthy() {
                        break;
                    }

                    for stmt in body.clone() {
                        if let RuntimeValue::Return(val) = self.execute_statement(stmt)? {
                            return Ok(RuntimeValue::Return(val));
                        }
                    }
                }
                Ok(RuntimeValue::Value)
            }

            Statement::ForStmt {
                variable,
                iterable,
                body,
            } => {
                let iterable_value = self.evaluate_expression(iterable)?;

                if let LiteralValue::Array(arr) = iterable_value {
                    for element in arr {
                        // Create new scope for loop variable
                        let parent_env = self.environment.clone();
                        let mut new_env = Environment::with_parent(parent_env);
                        new_env.define_variable(variable.clone(), element);
                        let old_env = std::mem::replace(&mut self.environment, new_env);

                        for stmt in body.clone() {
                            if let RuntimeValue::Return(val) = self.execute_statement(stmt)? {
                                self.environment = old_env;
                                return Ok(RuntimeValue::Return(val));
                            }
                        }

                        self.environment = old_env;
                    }
                } else {
                    return Err(anyhow!("Can only iterate over arrays"));
                }

                Ok(RuntimeValue::Value)
            }
        }
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<LiteralValue> {
        match expression {
            Expression::Literal { value } => Ok(value),

            Expression::Variable { name } => self
                .environment
                .get_variable(&name)
                .ok_or_else(|| anyhow!("Undefined variable: {}", name)),

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                self.apply_binary_operator(operator, left_val, right_val)
            }

            Expression::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(*operand)?;
                self.apply_unary_operator(operator, operand_val)
            }

            Expression::Call { callee, arguments } => {
                if let Expression::Variable { name } = *callee {
                    let function = self
                        .environment
                        .get_function(&name)
                        .ok_or_else(|| anyhow!("Undefined function: {}", name))?;

                    if arguments.len() != function.params.len() {
                        return Err(anyhow!(
                            "Function {} expects {} arguments, got {}",
                            name,
                            function.params.len(),
                            arguments.len()
                        ));
                    }

                    // Create new environment for function execution
                    let mut func_env = Environment::with_parent(self.environment.clone());

                    // Bind parameters
                    for (i, (param_name, param_type)) in function.params.iter().enumerate() {
                        let arg_value = self.evaluate_expression(arguments[i].clone())?;

                        if arg_value.get_type() != *param_type {
                            return Err(anyhow!(
                                "Argument type mismatch for parameter {}: expected {}, got {}",
                                param_name,
                                param_type,
                                arg_value.get_type()
                            ));
                        }

                        func_env.define_variable(param_name.clone(), arg_value);
                    }

                    // Execute function body
                    let old_env = std::mem::replace(&mut self.environment, func_env);

                    let mut return_value = LiteralValue::Int(0);
                    for stmt in function.body {
                        if let RuntimeValue::Return(val) = self.execute_statement(stmt)? {
                            return_value = val;
                            break;
                        }
                    }

                    self.environment = old_env;

                    // Type check return value
                    if return_value.get_type() != function.return_type {
                        return Err(anyhow!(
                            "Return type mismatch: expected {}, got {}",
                            function.return_type,
                            return_value.get_type()
                        ));
                    }

                    Ok(return_value)
                } else {
                    Err(anyhow!("Invalid function call"))
                }
            }

            Expression::MethodCall {
                object,
                method,
                arguments,
            } => {
                let object_name = if let Expression::Variable { name } = object.as_ref() {
                    name.clone()
                } else {
                    "unknown".to_string()
                };

                if let Expression::Variable { name } = *object
                    && name == "paneer"
                    && method == "bol"
                {
                    // Built-in print function
                    if arguments.len() != 1 {
                        return Err(anyhow!("paneer.bol() expects exactly 1 argument"));
                    }

                    let value = self.evaluate_expression(arguments[0].clone())?;
                    // Convert value to string for printing
                    let output = match value {
                        LiteralValue::Int(i) => i.to_string(),
                        LiteralValue::Float(f) => f.to_string(),
                        LiteralValue::Bool(b) => b.to_string(),
                        LiteralValue::String(s) => s,
                        LiteralValue::Array(arr) => {
                            let elements: Vec<String> = arr
                                .iter()
                                .map(|v| match v {
                                    LiteralValue::Int(i) => i.to_string(),
                                    LiteralValue::Float(f) => f.to_string(),
                                    LiteralValue::Bool(b) => b.to_string(),
                                    LiteralValue::String(s) => s.clone(),
                                    LiteralValue::Array(_) => "[nested array]".to_string(),
                                })
                                .collect();
                            format!("[{}]", elements.join(", "))
                        }
                    };
                    println!("{}", output);
                    return Ok(LiteralValue::Int(0));
                }

                Err(anyhow!("Unknown method: {}.{}", object_name, method))
            }

            Expression::ArrayLiteral { elements } => {
                let mut array_values = Vec::new();
                for element in elements {
                    array_values.push(self.evaluate_expression(element)?);
                }
                Ok(LiteralValue::Array(array_values))
            }

            Expression::ArrayAccess { array, index } => {
                let array_value = self.evaluate_expression(*array)?;
                let index_value = self.evaluate_expression(*index)?;

                if let (LiteralValue::Array(arr), LiteralValue::Int(idx)) =
                    (array_value, index_value)
                {
                    if idx < 0 || idx as usize >= arr.len() {
                        return Err(anyhow!("Array index out of bounds: {}", idx));
                    }
                    Ok(arr[idx as usize].clone())
                } else {
                    Err(anyhow!(
                        "Invalid array access: array must be array type and index must be int"
                    ))
                }
            }
        }
    }

    fn apply_binary_operator(
        &self,
        operator: BinaryOperator,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue> {
        match (operator, &left, &right) {
            // Arithmetic operations
            (BinaryOperator::Add, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                Ok(LiteralValue::Int(a + b))
            }
            (BinaryOperator::Add, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                Ok(LiteralValue::Float(a + b))
            }
            (BinaryOperator::Add, LiteralValue::String(a), LiteralValue::String(b)) => {
                Ok(LiteralValue::String(format!("{}{}", a, b)))
            }
            // String concatenation with automatic type conversion
            (BinaryOperator::Add, LiteralValue::String(a), right) => {
                let right_str = match right {
                    LiteralValue::Int(i) => i.to_string(),
                    LiteralValue::Float(f) => f.to_string(),
                    LiteralValue::Bool(b) => b.to_string(),
                    LiteralValue::String(s) => s.clone(),
                    LiteralValue::Array(arr) => {
                        let elements: Vec<String> = arr
                            .iter()
                            .map(|v| match v {
                                LiteralValue::Int(i) => i.to_string(),
                                LiteralValue::Float(f) => f.to_string(),
                                LiteralValue::Bool(b) => b.to_string(),
                                LiteralValue::String(s) => s.clone(),
                                LiteralValue::Array(_) => "[nested]".to_string(),
                            })
                            .collect();
                        format!("[{}]", elements.join(", "))
                    }
                };
                Ok(LiteralValue::String(format!("{}{}", a, right_str)))
            }
            (BinaryOperator::Add, left, LiteralValue::String(b)) => {
                let left_str = match left {
                    LiteralValue::Int(i) => i.to_string(),
                    LiteralValue::Float(f) => f.to_string(),
                    LiteralValue::Bool(b_val) => b_val.to_string(),
                    LiteralValue::String(s) => s.clone(),
                    LiteralValue::Array(arr) => {
                        let elements: Vec<String> = arr
                            .iter()
                            .map(|v| match v {
                                LiteralValue::Int(i) => i.to_string(),
                                LiteralValue::Float(f) => f.to_string(),
                                LiteralValue::Bool(b) => b.to_string(),
                                LiteralValue::String(s) => s.clone(),
                                LiteralValue::Array(_) => "[nested]".to_string(),
                            })
                            .collect();
                        format!("[{}]", elements.join(", "))
                    }
                };
                Ok(LiteralValue::String(format!("{}{}", left_str, b)))
            }

            (BinaryOperator::Subtract, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                Ok(LiteralValue::Int(a - b))
            }
            (BinaryOperator::Subtract, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                Ok(LiteralValue::Float(a - b))
            }

            (BinaryOperator::Multiply, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                Ok(LiteralValue::Int(a * b))
            }
            (BinaryOperator::Multiply, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                Ok(LiteralValue::Float(a * b))
            }

            (BinaryOperator::Divide, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                if *b == 0 {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(LiteralValue::Int(a / b))
                }
            }
            (BinaryOperator::Divide, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                if *b == 0.0 {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(LiteralValue::Float(a / b))
                }
            }

            // Comparison operations
            (BinaryOperator::Equal, _, _) => Ok(LiteralValue::Bool(left == right)),
            (BinaryOperator::NotEqual, _, _) => Ok(LiteralValue::Bool(left != right)),

            (BinaryOperator::Greater, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                Ok(LiteralValue::Bool(a > b))
            }
            (BinaryOperator::Greater, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                Ok(LiteralValue::Bool(a > b))
            }

            (BinaryOperator::Less, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                Ok(LiteralValue::Bool(a < b))
            }
            (BinaryOperator::Less, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                Ok(LiteralValue::Bool(a < b))
            }

            (BinaryOperator::GreaterEqual, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                Ok(LiteralValue::Bool(a >= b))
            }
            (BinaryOperator::GreaterEqual, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                Ok(LiteralValue::Bool(a >= b))
            }

            (BinaryOperator::LessEqual, LiteralValue::Int(a), LiteralValue::Int(b)) => {
                Ok(LiteralValue::Bool(a <= b))
            }
            (BinaryOperator::LessEqual, LiteralValue::Float(a), LiteralValue::Float(b)) => {
                Ok(LiteralValue::Bool(a <= b))
            }

            _ => Err(anyhow!(
                "Invalid binary operation: {} {:?} {}",
                left,
                operator,
                right
            )),
        }
    }

    fn apply_unary_operator(
        &self,
        operator: UnaryOperator,
        operand: LiteralValue,
    ) -> Result<LiteralValue> {
        match (operator, operand) {
            (UnaryOperator::Minus, LiteralValue::Int(value)) => Ok(LiteralValue::Int(-value)),
            (UnaryOperator::Minus, LiteralValue::Float(value)) => Ok(LiteralValue::Float(-value)),
            (UnaryOperator::Not, value) => Ok(LiteralValue::Bool(!value.is_truthy())),
            _ => Err(anyhow!("Invalid unary operation")),
        }
    }
}
