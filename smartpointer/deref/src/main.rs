// fn main()  
// {  
//   let a = 20;  
//   let b = &a;  
//   if a==*b  
//   {  
//     println!("a and *b are equal");  
//   }  
    
//   else  
//   {  
//     println!("they are not equal");  
//   }  
// }  

fn main()  
{  
  let a = 11;  
  let b = Box::new(a);  
  print!("Value of *b is {}",*b);  
}  
