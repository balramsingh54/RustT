// struct Example  
// {  
//   a : i32,  
//  }  
//       impl Drop for Example  
// {  
//   fn drop(&mut self)  
//   {  
//     println!("Dropping the instance of Example with data : {}", self.a);  
//   }  
// }  
//       fn main()  
// {  
//   let a1 = Example{a : 10};  
//   let b1 = Example{a: 20};  
//   println!("Instances of Example type are created");  
// }  



struct Example  
{  
  a : String,  
}  
  
impl Drop for Example  
{  
  fn drop(&mut self)  
  {  
    println!("Dropping the instance of Example with data : {}", self.a);  
  }  
}  
  
fn main()  
{  
  let a1 = Example{a : String::from("Hello")};  
  drop(a1);  
  let b1 = Example{a: String::from("World")};  
  println!("Instances of Example type are created");  
}  