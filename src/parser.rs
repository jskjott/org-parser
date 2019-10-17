use crate::scanner::Token;
use crate::scanner::TokenType;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Node {
    pub data: Vec<Token>,
    pub children: HashMap<String, Node>,
}

pub fn parse(source: Vec<Token>) -> Node {
    to_dom(source)
}

fn till_next(token_list: Vec<Token>, length: usize) -> Vec<Token> {
    let mut tokens = vec![];

    let mut asterisk = false;
    let mut i = 1;

    while i < token_list.len() && !asterisk && token_list[i].token_type != TokenType::EOF {
        if token_list[i].token_type == TokenType::Asterisk && token_list[i].lexeme.len() == length {
            asterisk = true;
        } else {
            tokens.push(token_list[i].clone());
        }

        i = i + 1;
    }

    tokens
}

fn to_dom(source: Vec<Token>) -> Node {
    let mut dom: Node = Node {
        data: vec![],
        children: HashMap::new(),
    };
    let mut data = vec![];
    let mut children = vec![];

    let mut active: isize = -1;

    for (i, token) in source.iter().enumerate() {
        let lexeme_length = token.lexeme.len();

        if i as isize > active {
            if token.token_type == TokenType::Asterisk {
                let (_left, right) = source.split_at(i);
                let right = right.to_vec();
                let inner_source = till_next(right, lexeme_length);

                if (i + inner_source.len()) as isize > active {
                    active = (i + inner_source.len()) as isize;
                }

                let mut sub_data: Vec<Token> = vec![token.clone()];

                let mut z = 0;

                while z < inner_source.len() && inner_source[z].line == token.line {
                    sub_data.push(inner_source[z].clone());
                    z = z + 1;
                }

                let (_left, right) = inner_source.split_at(z);
                let mut sub_dom = to_dom(right.to_vec());

                let mut child: Node = Node {
                    data: vec![],
                    children: HashMap::new(),
                };

                for (i, token) in sub_dom.data.iter().enumerate() {
                    if i > 0 && sub_dom.data[i - 1].line != token.line {
                        let mut node_title: String = "".to_owned();

                        for token in child.data.clone() {
                            node_title.push_str(&token.lexeme);
                        }

                        sub_dom.children.insert(node_title, child);
                        child = Node {
                            data: vec![],
                            children: HashMap::new(),
                        };
                    } else {
                        child.data.push(token.clone())
                    }
                }

                let inner_element: Node = Node {
                    data: sub_data,
                    children: sub_dom.children,
                };

                children.push(inner_element)
            } else {
                let mut z = i;

                while z < source.len() && source[z].line == token.line {
                    data.push(source[z].clone());

                    z = z + 1;
                }

                active = (z - 1) as isize;
            }
        }
    }

    for node in children {
        let mut node_title: String = "".to_owned();

        for token in node.data.clone() {
            if token.token_type == TokenType::String {
                node_title.push_str(&token.lexeme);
            }
        }

        dom.children.insert(node_title, node);
    }

    dom.data = data;

    dom
}
