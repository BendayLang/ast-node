#![allow(dead_code, unused_variables)]
mod ast;
mod return_value;

pub use ast::{ASTNode, ASTNodeData, Function, If, IfElse, VariableAssignment, While};
pub use return_value::ReturnValue;

#[cfg(debug_assertions)]
pub use ast::ast_example;
