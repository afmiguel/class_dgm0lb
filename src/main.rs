enum LinkedList{
    Nil,
    Node(i32, Box<LinkedList>),
}

impl LinkedList{
    pub fn display_list(&self){
        match self{
            LinkedList::Nil => println!("nil"),
            LinkedList::Node(value, tail ) => {
                print!("{:?}->", value);
                tail.display_list();
            }
        }
    }
}

use LinkedList::{Nil, Node};

fn main() {
    let lista = Node(10, Box::new(Node(20, Box::new(Nil))));
}
