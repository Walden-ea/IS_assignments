use std::{collections::{VecDeque, HashSet}, fmt::Display, hash::Hash, rc::Rc};

#[derive(Hash, Clone, Debug)]
struct Node<T:PartialEq> {
    pub val: T,
    pub parent: Option<Rc<Node<T>>>
}
impl<T:PartialEq> PartialEq for Node<T>{
    fn eq(&self, other: &Self) -> bool{
        return self.val == other.val;
    }
}
impl<T:PartialEq> Eq for Node<T>{}

type Deque = VecDeque<Node<i32>>;
type Set = HashSet<Node<i32>>;



fn simple_solve(given:i32,goal:i32) -> Option<Node<i32>>{
    let root = Node{val: given,parent: None};
    let mut open = VecDeque::from([root]);
    let mut closed = HashSet::from([given]);

    while let Some(node) = open.pop_back(){

        if node.val == goal
        { return Some(node)}
        else {
            let rc = Rc::new(node);
            let mut populate = |f: fn(&i32)->i32| { 
                if f(&rc.val) <= goal && !closed.contains(&f(&rc.val)) {
                    open.push_front(Node{val:f(&rc.val), parent: Some(Rc::clone(&rc))});
                    closed.insert(f(&rc.val));
                }};
            populate(|x| x+3);
            populate(|x| x*2);
               
        }
    }
    None
}

fn slightly_different_solve(given:i32,goal:i32) -> Option<Node<i32>>{
    let root = Node{val: given,parent: None};
    let mut open = VecDeque::from([root]);
    let mut closed = HashSet::from([given]);

    while let Some(node) = open.pop_back(){

        if node.val == goal
        { return Some(node)}
        else {
            let rc = Rc::new(node);
            let mut populate = |f: fn(&i32)->i32| { 
                if /*  && f(&rc.val)>given && f(&rc.val)<=goal */!closed.contains(&f(&rc.val)) {
                    open.push_front(Node{val:f(&rc.val), parent: Some(Rc::clone(&rc))});
                    closed.insert(f(&rc.val));
                }};
            populate(|x| x+3);
            populate(|x| x*2);
            populate(|x| x-2);
               
        }
    }
    None
}

fn backwards_solve(given:i32,goal:i32) -> Option<Node<i32>>{
    let root = Node{val: goal,parent: None};
    let mut open = VecDeque::from([root]);
    let mut closed = HashSet::from([goal]);

    while let Some(node) = open.pop_back(){

        if node.val == given
        { return Some(node)}
        else {
            let rc = Rc::new(node);
            if &rc.val - 3 >=given && !closed.contains(&(&rc.val-3)){
                open.push_front(Node{val:&rc.val-3, parent: Some(Rc::clone(&rc))});
                closed.insert(&rc.val-3);
            } 
            if &rc.val%2==0 && &rc.val / 2 >=given && !closed.contains(&(&rc.val/2)){
                open.push_front(Node{val:&rc.val/2, parent: Some(Rc::clone(&rc))});
                closed.insert(&rc.val/2);
            }    
        }
    }
    None
}

fn uber_solve(given:i32,goal:i32) -> Option<Node<i32>>{
    if given == goal {
        return Some(Node{val:given,parent:None});
    }


    let root_front = Node{val: given,parent: None};
    let root_back = Node{val: goal,parent: None};

    let mut open_front = VecDeque::from([root_front]);
    let mut closed_front = HashSet::from([given]);
    let mut open_back = VecDeque::from([root_back]);
    let mut closed_back = HashSet::from([goal]);
    let mut front = HashSet::new();
    let mut back =  HashSet::from([Rc::new(Node{val: goal,parent: None})]);

    loop {
        let open = &mut open_front;
        let closed = &mut closed_front;

        let node = open.pop_back()?; 
        for back_node in &back {
            if back_node.val == node.val {
                return Some(glue_nodes(Rc::new(node), Rc::clone(back_node)));
            }
        }
        
        // оно признак завершения потому что на предыдущей итерации мы популейтили деку
        front.insert(Rc::new(node));
        while let Some(front_node) = open.pop_back(){
            for back_node in &back {
                if back_node.val == front_node.val {
                    return Some(glue_nodes(Rc::new(front_node), Rc::clone(back_node)));
                }
            }
            front.insert(Rc::new(front_node));   
        }


        for node in &front {
            let rc = node;
            let mut populate = |f: fn(&i32)->i32| { 
                if  f(&rc.val)>given && f(&rc.val)<=goal && !closed.contains(&f(&rc.val)) {
                    open.push_front(Node{val:f(&rc.val), parent: Some(Rc::clone(&rc))});
                    closed.insert(f(&rc.val));
                }};
            populate(|x| x+3);
            populate(|x| x*2);
        }
        
        //развернули начало

        

        let open = &mut open_back;
        let closed = &mut closed_back;



        while let Some(back_node) = open.pop_back(){
            for front_node in &front { //видимо придется юзать референс каунт для перекувырков между сетом и декой((
                if back_node.val == front_node.val {
                    return Some(glue_nodes(front_node.clone(), Rc::new(back_node)));
                }
            }
            back.insert(Rc::new(back_node));
        } 

        for node in &back {
            let rc = node;
            let mut populate = |f: fn(&i32)->i32| { 
                if  f(&rc.val)>given && f(&rc.val)<=goal && !closed.contains(&f(&rc.val)) {
                    open.push_front(Node{val:f(&rc.val), parent: Some(Rc::clone(&rc))});
                    closed.insert(f(&rc.val));
                }};
                populate(|x| x+3);
                populate(|x| x*2);
        }
    }
}


fn main() {
    unwrap1(uber_solve(2,10000001));
}

fn unwrap1(root: Option<Node<i32>>){
    match root{
    Some(root) => {
        let mut count = 0;
        print!("{} ",root.val);
        let mut root = &root.parent;
        while let Some(node) = &root{
            count+=1;
            print!("-> {} ",node.val);
            root = &node.parent;
        }
        println!("\ncount: {}",count);
    }
    None => println!("None found!")}
}

fn glue_nodes(front: Rc<Node<i32>>,back: Rc<Node<i32>>) -> Node<i32>{
    println!("hello from glue nodes");
    if let Some(parent) = &front.parent{
        let mut root = Node{val:back.val,parent:Some(Rc::clone(&parent))};
        while let Some(back) = &back.parent
        {
            root = Node{val:back.val,parent:Some(Rc::new(root))};      
        }
    return root;
}
    return (*back).clone();
}

