pub struct Node {
    pub item: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub fn pre_order(root: Node) {
    po(Some(Box::new(root)));
    println!()
}

fn po<T: AsRef<Node>>(node: Option<T>) {
    if let Some(n) = node {
        let n: &Node = <T as AsRef<Node>>::as_ref(&n);
        print!("{} ", &n.item);
        po(n.left.as_ref());
        po(n.right.as_ref());
    }
}
