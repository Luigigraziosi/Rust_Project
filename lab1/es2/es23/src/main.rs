struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            size: 0,
            count: 0,
        }
    }

    fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
    
    fn count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }
    fn to_string(&self) -> String{
        format!("name:{} size:{} count:{}", self.name, self.size, self.count)
    }
    fn grow(&mut self){
        self.size+=1
    }
    fn inc(&mut self){
        self.count+=1
    }
}

fn main() {
    let mut node = Node::new(String::from("nodo")).size(10).count(5);
    println!("{}", node.to_string());
    node.grow();
    node.inc();
    println!("{}", node.to_string());
}

