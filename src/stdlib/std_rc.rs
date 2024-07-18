use std::cell::RefCell;
use std::rc::{Rc, Weak};


#[test]
fn rc(){
    let rc1 = Rc::new(5);
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    println!("Reference count: {}",Rc::strong_count(&rc1));
    println!("Values: rc1={}, rc2={}, rc3={}", rc1, rc2, rc3);

    let rc_weak = Rc::downgrade(&rc1);
}

#[derive(Debug)]
struct Node<T>{
    value :T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}
impl<T> Node<T>{
    fn new(value:T)->Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node{
            value,
            next: None,
            prev: None,
        }))
    }
}
struct DoublyLinkedList<T>{
    head : Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T>{
    fn new()->Self{
        DoublyLinkedList{
            head:None,
            tail:None,
        }
    }
    fn append(&mut self,value: T){
        let new_node = Node::new(value);
        let rc_new_node = Rc::clone(&new_node);
        match self.tail.take() {
            Some(tail)=>{
                tail.borrow_mut().next = Some(rc_new_node);
                new_node.borrow_mut().prev = Some(Rc::downgrade(&tail));
                self.tail = Some(new_node);
            }
            None =>{
                self.head = Some(rc_new_node);
                self.tail = Some(new_node);
            }
        }
    }
    fn prepend(&mut self,value:T){
        let new_node = Node::new(value);
        let rc_new_node = Rc::clone(&new_node);
        match self.head.take() {
            Some(head)=>{
                head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(Rc::clone(&head));
                self.head = Some(new_node);
            }
            None=>{
                self.head = Some(rc_new_node);
                self.tail =  Some(new_node);
            }
        }
    }

    fn delete(&mut self,value:T)
    where
        T: PartialEq,
    {
        let mut current = self.head.clone();
        while let Some (node)=current{
            if node.borrow().value== value{
                let prev = node.borrow().prev.clone();
                let next = node.borrow().next.clone();

                if let Some(prev)=prev.clone().and_then(|w|w.upgrade()){
                    prev.borrow_mut().next = next.clone();
                }else{
                    self.head = next.clone();
                }

                if let Some(next)= next{
                    next.borrow_mut().prev = prev;
                }else{
                    self.tail = prev.and_then(|w| w.upgrade());
                }

                break;
            }
            current = node.borrow().next.clone();
        }
    }

    fn print_list(&self)
    where
        T:std::fmt::Debug,
    {
        let mut current = self.head.clone();
        while let Some(node) = current{
            println!("{:?}",node.borrow().value);
            current = node.borrow().next.clone();
        }
    }
}

// Rc::downgrade 和 Weak::upgrade 的用途
// Rc::downgrade
// Rc::downgrade方法用于将Rc指针转换为Weak指针。Weak指针不会增加引用计数，因此可以用来打破循环引用，避免内存泄漏。Weak指针是非拥有型指针，当所有的Rc指针都被丢弃时，Weak指针不会阻止内存的释放。
//
// Weak::upgrade
// Weak::upgrade方法用于尝试将Weak指针转换回Rc指针。如果底层值还未被释放，upgrade方法将返回Some(Rc<T>)，否则返回None。这使得我们可以安全地访问可能已经被释放的值。

#[test]
fn linked_list(){
    let mut list = DoublyLinkedList::new();
    list.append(1);
    list.append(2);

    list.prepend(3);
    list.print_list();
    list.delete(3);
    println!("after delete...");
    list.print_list();
}

