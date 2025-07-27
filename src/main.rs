#[derive(Debug)]
pub struct Node{
    pub data: i32,
    pub next: Option<Box<Node>>,
}

pub struct LinkedList{
    pub head: Option<Box<Node>>,
}

impl LinkedList{
    pub fn new(data: i32)-> Self{
        Self { head: Some(Box::new(Node{data, next: None})) }
    }

    pub fn push_back(&mut self, data: i32) {
        let mut current = &mut self.head;
        while let Some(next_node) = current{
            current = &mut next_node.next;
        }
        *current = Some(Box::new(Node{data, next:None}));
    }

    pub fn print(&self) {
        let mut current = self.head.as_ref();

        while let Some(ref node) = current{
            println!("{}", node.data);
            current = node.next.as_ref();
        }
    }
}
fn main() {
    let mut link = LinkedList::new(10);
   link.push_back(20);
   link.push_back(30);
   link.print();
}