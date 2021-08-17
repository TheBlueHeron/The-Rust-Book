use crate::List::{Cons, Nil};
use crate::MultiOwnerList::{Cns, Nl};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

fn main() {
    let b = Box::new(5); // 5 is stored on the heap; the box (pointer) data is stored on the stack
    // box implements only Deref trait and Drop trait and only provides indirection and heap allocation
    println!("b = {}", b);

    // recursive list using box 'indirection' to get a recursive type with known size
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // use dereferencing to follow pointer to value; works on boxes too

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereferencing possible, because Deref trait is implemented

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion also works!

    hello(&(*m)[..]); // syntax without deref coercion!

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    //c.drop(); // not allowed
    drop(c); // std::mem::drop(_x: T)
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Using Rc<T> to Share Data
    let a = Rc::new(Cns(5, Rc::new(Cns(10, Rc::new(Nl)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cns(3, Rc::clone(&a)); // no deep clone is made, only reference count is increased
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cns(4, Rc::clone(&a)); // b and c share ownership of a, but immutably!
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Interior Mutability: A Mutable Borrow to an Immutable Value
    // see lib.rs

    // Using Weak<T> to prevent reference cycles and memory leaks
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // the Node in leaf now has two owners: leaf and branch
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // use weak reference to branch as parent for leaf

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // The lack of infinite output here indicates that this code doesn’t create a reference cycle
        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // leaf.parent has a weak reference to branch
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // branch has a strong reference stored in branche.children, so strong count = 2
    } // branch node is dropped here, because the weak count of 1 from leaf.parent has no bearing on whether or not Node is dropped -> no memory leaks!
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // The parent is now None again
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

// Cons list definition -> won't compile because of infinite size
enum List {
    // Cons(i32, List), // recursive variant with undeterminable size
    Cons(i32, Box<List>), // recursive variant with determinable size (size = i32 size + size of pointer to boxed item on the heap, i.e. a usize)
    Nil, // the non-recursive variant that signals the end of the list
}

enum MultiOwnerList {
    Cns(i32, Rc<MultiOwnerList>),
    Nl,
}

struct MyBox<T>(T); // MyBox is a tuple struct with one element of type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // defines an associated type for the Deref trait to use (see Chapter 19 Advanced Features)

    fn deref(&self) -> &Self::Target {
        &self.0 // return a reference to the value we want to access with the * operator; no move takes place
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>, // internally mutable collection of nodes that can have multiple ownership
}
