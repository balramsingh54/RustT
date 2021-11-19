// data is stored in heap

// box <T> type in pointer in enum
// enum list{
//     cons(i32,List),
//     Null
// }
// use List::{Cons, Nil};  


// fn main()  
// {  
//   let a = Box :: new(1);  
//   print!("value of a is : {}",a);  
// }   


// enum List {  
//     Cons(i32, List),  
//     Nil,  
// }  
// use List::{Cons, Nil};  
// fn main()  
// {  
//   let list = List::Cons(1,Cons(2,Cons(3,Nil)));  
//   for i in list.iter()  
//   {  
//     print!("{}",i);  
//   }  
// }  

#[derive(Debug)]   
enum List {  
    Cons(i32, Box<List>),  
    Nil,  
}  
use List::{Cons, Nil};  
fn main()  
{  
  let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));  
    
    print!("{:?}",list);  
    
}  