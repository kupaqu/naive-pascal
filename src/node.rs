use super::token::*;

#[derive(Debug)]
pub struct Node {
    /* использую вектор для избежания использования std::boxed::Box
       и сопутствующих сложностей его использования */
    pub children: Vec<Node>,
    pub token: Token
}

impl Node {
    /* конструкторы */
    pub fn bin_op(left: Node, my_token: Token, right: Node) -> Node {
        println!("binop {:?}", my_token);
        Node {
            children: vec![left, right],
            token: my_token
        }
    }
    pub fn un_op(my_token: Token, my_child: Node) -> Node {
        println!("unop {:?}", my_token);
        Node {
            children: vec![my_child],
            token: my_token
        }
    }
    pub fn number(my_token: Token) -> Node {
        println!("number {:?}", my_token);
        Node {
            children: Vec::new(),
            token: my_token
        }
    }
}