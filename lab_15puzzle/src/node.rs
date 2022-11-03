use std::{rc::Rc, fmt::Display, fmt};

#[derive(Hash, Clone, Debug)]
pub struct Node<T:PartialEq> {
    pub val: T,
    pub parent: Option<Rc<Node<T>>>
}
impl<T:PartialEq> PartialEq for Node<T>{
    fn eq(&self, other: &Self) -> bool{
        return self.val == other.val;
    }
}
impl<T:PartialEq> Eq for Node<T>{}

impl <T:Display + PartialEq> Display for Node<T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut count = 0;
        let mut nodes = format!("{} -> ",self.val);
        let mut root = &self.parent;
        while let Some(node) = &root{
            count+=1;
            if let Some(n) = &node.parent {
                nodes.push_str(&format!("{} -> ", node.val));
            }
            else {
                nodes.push_str(&format!("{}", node.val));
            }
            root = &node.parent;
        }
        let mut res = format!("\ncount: {count}\n");
        res.push_str(&nodes);
        write!(f,"{}",res)}
}

