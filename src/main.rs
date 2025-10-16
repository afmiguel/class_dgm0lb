enum LinkedList{
    Nil,
    Node(i32, Box<LinkedList>),
}


fn main() {
    let lista = LinkedList::Node(10, Box::new(LinkedList::Nil));
}
