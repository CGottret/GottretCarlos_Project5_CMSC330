use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Helmet(bool),              //is damaged?
    LeftThrusters(bool, i32),  //is damaged? How much power left?
    RightThrusters(bool, i32), //is damaged? How much power left?
    LeftRepulsor(bool, i32),   //is damaged? How much power left?
    RightRepulsor(bool, i32),  //is damaged? How much power left?
    ChestPiece(bool, i32),     //is damaged? How much power left?
    Missiles(i32),             //how many missiles left?
    ArcReactor(i32),           // How much power left?
    Wifi(bool),                // connected to wifi?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Armor {
    pub component: Component,
    pub version: i32,
}

// Part 2

// Students should fill in the Link type themselves. The Node and List types are given as is.
type Link = Option <Arc<RwLock<Node>>>;

struct Node {
    data: Armor,
    rest: Link,
}

#[derive(Clone)]
pub struct List {
    head_link: Link,
    size: usize,
}

impl List {
    
    pub fn new() -> Self {
        List{
            head_link: None,
            size: 0,
        }  
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn peek(&self) -> Option<Armor> {        
        if self.size == 0 {
            return None
        } else {
            let head = &self.head_link;
            let as_ref = head.as_ref();
            return as_ref.map(|item| item.try_read().unwrap().data)
        }
    }

    pub fn push(&mut self, component: Armor) {
        let node = Node {
            data: component,
            rest: self.head_link.clone(),
        }; 
        let new_node = Arc::new(RwLock::new(node));
        self.head_link = Some(new_node);
        self.size+=1;
    }

    pub fn pop(&mut self) -> Option<Armor> {
        if self.size == 0 {
            return None
        }
        else {
            let val = self.peek();

            let head = &self.head_link;
            let as_ref = head.as_ref();

            self.head_link = as_ref.map(|item| item.try_read().unwrap().rest.clone()).unwrap();
            self.size-=1;

            return val;
        }
    }
}

pub fn repair_matcher(component: Component) -> Component {
    match component {
        Component::LeftThrusters (true, num) => {return Component::LeftThrusters(false, 100);}
        Component::LeftRepulsor (true, num) => {return Component::LeftRepulsor(false, 100);}
        Component::RightRepulsor (true, num) => {return Component::RightRepulsor(false, 100);}  
        Component::RightThrusters (true, num) => {return Component::RightThrusters(false, 100);}
        Component::ChestPiece (true, num) => {return Component::ChestPiece(false, 100);}
        _ => return component,
    }
}

// Part 3

#[derive(Clone)]
pub struct Suit {
    pub armor: List,
    pub version: i32,
}

impl Suit {
    pub fn is_compatible(&self) -> bool {
        let mut armorr = self.armor.clone();
        let vers = self.version;

        for i in 0..armorr.size() {
            if armorr.peek().unwrap().version != vers  {
                return false;
            }
            armorr.pop();
        }
        return true;
    }

    pub fn repair(&mut self) {
        let mut armorr = self.armor.clone();

        for i in 0..armorr.size() {
            let head = &armorr.head_link;
            if let Some(head) = head  {
                let mut head = head.try_write().unwrap();
                let comp = head.data.component;
                let new_comp = repair_matcher(comp);
                head.data.component = new_comp;
            }
            armorr.pop();
        }
    }
}
