#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_value(&mut self, data: T) {
       self.head = Some(Box::new(Node {
           next: self.head.take(),
           data: data
       })) 
    }

    fn get_value(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
            None => None
        }
    }
}

fn main() {
    let mut list_of_nums = LinkedList::<u8>::new();
    list_of_nums.push_value(1);
    list_of_nums.push_value(2);
    list_of_nums.push_value(3);

    println!("List {:?}", list_of_nums);
    println!("Nums: {:?}", list_of_nums.get_value());
    println!("List {:?}", list_of_nums);

    let mut list_of_nums_2 = LinkedList::<&str>::new();
    list_of_nums_2.push_value("golf");
    list_of_nums_2.push_value("baseball");
    list_of_nums_2.push_value("chess");

    println!("List2 {:?}", list_of_nums_2);
    println!("Nums of list 2: {:?}", list_of_nums_2.get_value());
    println!("List2 {:?}", list_of_nums_2);
}