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
    
    pub fn push_front(&mut self, data: i32) {
        let new_node = Box::new(Node{data, next: self.head.take()});
        self.head = Some(new_node);
    }
    
    pub fn pop_front(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }
    
    pub fn pop_back(&mut self){ 
        let mut current = &mut self.head;
        
        if current.is_none(){
            return;
        }
        
        if current.as_ref().unwrap().next.is_none(){
            *current = None;
            return;
        }
        // going to end of linklist
        while let Some(node) = current{
            if node.next.is_some() && node.next.as_ref().unwrap().next.is_none() {
                node.next = None;
            }
            current = &mut node.next;
        }
    }
}

fn main() {
    let mut link = LinkedList::new(10);
   link.push_back(20);
   link.push_back(30);
   link.print();
   link.push_front(100);
   link.print();
   link.pop_front();
   println!("After pop front");
   link.pop_back();
   link.pop_back();
   println!("After pop back");
   link.print();
}