use std::cell::RefCell;  
fn main()  
{  
  let a = RefCell::new(15);  
  let b = a.borrow();  
  let c = a.borrow();  
  println!("Value of b is : {}",b);  
  println!("Value of c is : {}",c);  
}  