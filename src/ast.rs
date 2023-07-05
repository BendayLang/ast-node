use std::fmt;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Sequence(Vec<ASTNode>),
    While(bool, Box<ASTNode>, Vec<ASTNode>),
    IfElse(IfElse),
    Value(String),
    VariableAssignment(String, Box<ASTNode>),
    FunctionCall(Function),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnValue {
    String_(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    None,
}

impl fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnValue::String_(str) => write!(f, "{str}"),
            ReturnValue::Int(val) => write!(f, "{val}"),
            ReturnValue::Float(val) => write!(f, "{val}"),
            ReturnValue::Bool(val) => write!(f, "{val}"),
            ReturnValue::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<ASTNode>,
    pub sequence: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct IfElse {
    pub if_: If,
    pub elif: Option<Vec<If>>,
    pub else_: Option<If>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub is_builtin: bool,
    pub argv: Vec<ASTNode>,
}

/* ********************** */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ast_executor_test() {
        let ast = ASTNode::Sequence(vec![
            ASTNode::VariableAssignment(
                "age de Bob".to_string(),
                Box::new(ASTNode::Value("6".to_string())),
            ),
            ASTNode::While(
                false,
                Box::new(ASTNode::Value("{age de Bob} < 13".to_string())),
                vec![
                    ASTNode::VariableAssignment(
                        "age de Bob".to_string(),
                        Box::new(ASTNode::Value("{age de Bob} + 1".to_string())),
                    ),
                    ASTNode::FunctionCall(Function {
                        name: "print".to_string(),
                        is_builtin: true,
                        argv: vec![ASTNode::Value(
                            "Bravo Bob ! tu as maintenant {age de Bob} ans !".to_string(),
                        )],
                    }),
                ],
            ),
            ASTNode::FunctionCall(Function {
                name: "print".to_string(),
                is_builtin: true,
                argv: vec![ASTNode::Value(
                    "Bob est parti a l'age de {age de Bob} !".to_string(),
                )],
            }),
        ]);
    }
}
