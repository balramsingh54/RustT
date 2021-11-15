// //When a program tries to access the invalid reference is known as a Dangling reference.

// fn main() {
//     let a = 5;
//     let b= &a;
//     println!("value of b is {}", b);
// }


fn main()  
{  
  let a;  
  {  
    let b = 10;  
     a = &b;  
     println!("a : {}",a);  

  }  
//   println!("a : {}",a);  
} 