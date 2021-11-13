// fn main()  
// {  
//   let a = vec![1,2,3,4,5];  
//   let b = vec![2.3,3.3,4.3,5.3];  
//   let result = add(&a);  
//   let result1 = add(&b);  
//   println!("The value of result is {}",result);  
//   println!("The value of result1 is {}",result1);  
// }  
  
// fn add<T>(list:&[T])->T  
// {  
//   let mut c =0;  
//   for &item in list.iter()  
//   {  
//     c= c+item;  
//   }  

struct Value<T>  
{  
  a:T,  
  b:T,  
}  
fn main()  
{  
  let integer = Value{a:2,b:3};  
  let float = Value{a:7.8,b:12.3};  
  println!("integer values : {},{}",integer.a,integer.b);  
  println!("Float values :{},{}",float.a,float.b);  
}  

// struct Value<T>  
// {  
//   a:T,  
//   b:T,  
// }       
// fn main()  
// {  
//   let c = Value{a:2,b:3.6};  
//   println!("c values : {},{}",c.a,c.b);  
//  }   
//  this will give an error with data type 