#[derive(Clone)]
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

    pub fn push_back(&mut self, value: i32){
        match self{
            LinkedList::Nil => *self = LinkedList::Node(value, Box::new(LinkedList::Nil)),
            LinkedList::Node(v, tail) => tail.push_back(value),
        }
    }

    pub fn push_front(&mut self, value: i32){
        *self = LinkedList::Node(value, Box::new(self.clone()));
    }
}

use LinkedList::{Nil, Node};

fn main() {
    let mut lista = Nil;
    println!("Criando a lista...");
    lista.push_back(10);
    lista.push_back(20);
    lista.push_back(30);
    lista.display_list();

    println!("push_front(5)");
    lista.push_front(5);
    lista.display_list();
}

//Testes 123