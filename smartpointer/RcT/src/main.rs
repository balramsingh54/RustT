enum List   
{  
  Cons(i32, Rc<List>),  
  Nil,  
}  
use List::{Cons,Nil};  
use std::rc::Rc;  
fn main()  
{  
  let a = Rc::new(Cons(10, Rc::new(Cons(15,Rc::new(Nil)))));  
  let b = Cons(2, Rc::clone(&a));  
  let c = Cons(1, Rc::clone(&a));   
}