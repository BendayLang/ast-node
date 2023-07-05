#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ASTNode {
    pub id: u32,
    #[serde(flatten)]
    pub data: ASTNodeData,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum ASTNodeData {
    #[serde(rename = "sequence")]
    Sequence(Sequence),
    #[serde(rename = "while")]
    While(While),
    #[serde(rename = "ifElse")]
    IfElse(IfElse),
    #[serde(rename = "input")]
    Input(String),
    #[serde(rename = "variableAssignment")]
    VariableAssignment(VariableAssignment),
    #[serde(rename = "functionCall")]
    FunctionCall(Function),
}

type Sequence = Vec<ASTNode>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct While {
    #[serde(rename = "isDo")]
    pub is_do: bool,
    pub condition: Box<ASTNode>,
    pub sequence: Vec<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct If {
    pub condition: Box<ASTNode>,
    pub sequence: Vec<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct IfElse {
    #[serde(rename = "if")]
    pub if_: If,
    pub elif: Option<Vec<If>>,
    #[serde(rename = "else")]
    pub else_: Option<If>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VariableAssignment {
    pub name: String,
    pub value: Box<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Function {
    pub name: String,
    #[serde(rename = "isBuiltin")]
    pub is_builtin: bool,
    pub argv: Vec<ASTNode>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ast_executor_test() {
        let expected_json = r#"{
            "id": 0,
            "type": "sequence",
            "data": [
              {
                "id": 1,
                "type": "variableAssignment",
                "data": {
                  "name": "age de Bob",
                  "value": { "id": 2, "type": "input", "data": "6" }
                }
              },
              {
                "id": 3,
                "type": "while",
                "data": {
                  "isDo": false,
                  "condition": { "id": 4, "type": "input", "data": "{age de Bob} < 13" },
                  "sequence": [
                    {
                      "id": 5,
                      "type": "variableAssignment",
                      "data": {
                        "name": "age de Bob",
                        "value": { "id": 6, "type": "input", "data": "{age de Bob} + 1" }
                      }
                    },
                    {
                      "id": 7,
                      "type": "functionCall",
                      "data": {
                        "name": "print",
                        "isBuiltin": true,
                        "argv": [
                          {
                            "id": 8,
                            "type": "input",
                            "data": "Bravo Bob ! tu as maintenant \"{age de Bob}\" ans !"
                          }
                        ]
                      }
                    }
                  ]
                }
              },
              {
                "id": 9,
                "type": "functionCall",
                "data": {
                  "name": "print",
                  "isBuiltin": true,
                  "argv": [
                    {
                      "id": 10,
                      "type": "input",
                      "data": "Bob est parti a l'age de {age de Bob} !"
                    }
                  ]
                }
              }
            ]
          }
          "#;
        let ast_from_str: ASTNode = serde_json::from_str(expected_json).unwrap();

        assert_eq!(ast_example(), ast_from_str);
    }
}

pub fn ast_example() -> ASTNode {
    ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![
            ASTNode {
                id: 1,
                data: ASTNodeData::VariableAssignment(VariableAssignment {
                    name: "age de Bob".to_string(),
                    value: Box::new(ASTNode {
                        id: 2,
                        data: ASTNodeData::Input("6".to_string()),
                    }),
                }),
            },
            ASTNode {
                id: 3,
                data: ASTNodeData::While(While {
                    is_do: false,
                    condition: Box::new(ASTNode {
                        id: 4,
                        data: ASTNodeData::Input("{age de Bob} < 13".to_string()),
                    }),
                    sequence: vec![
                        ASTNode {
                            id: 5,
                            data: ASTNodeData::VariableAssignment(VariableAssignment {
                                name: "age de Bob".to_string(),
                                value: Box::new(ASTNode {
                                    id: 6,
                                    data: ASTNodeData::Input("{age de Bob} + 1".to_string()),
                                }),
                            }),
                        },
                        ASTNode {
                            id: 7,
                            data: ASTNodeData::FunctionCall(Function {
                                name: "print".to_string(),
                                is_builtin: true,
                                argv: vec![ASTNode {
                                    id: 8,
                                    data: ASTNodeData::Input(
                                        "Bravo Bob ! tu as maintenant \"{age de Bob}\" ans !"
                                            .to_string(),
                                    ),
                                }],
                            }),
                        },
                    ],
                }),
            },
            ASTNode {
                id: 9,
                data: ASTNodeData::FunctionCall(Function {
                    name: "print".to_string(),
                    is_builtin: true,
                    argv: vec![ASTNode {
                        id: 10,
                        data: ASTNodeData::Input(
                            "Bob est parti a l'age de {age de Bob} !".to_string(),
                        ),
                    }],
                }),
            },
        ]),
    }
}
