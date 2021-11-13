trait HasArea  
{  
  fn area(&self)->f64;  
}  
struct Triangle  
{  
  base : f64,  
  height : f64,  
}  
  
impl HasArea for Triangle  
{  
  fn area(&self)->f64  
  {  
    0.5*(self.base*self.height)  
  }  
}  
struct Square  
{  
  side : f64,  
}  
  
impl HasArea for Square  
{  
  fn area(&self)->f64  
  {  
     self.side*self.side  
  }  
}  
fn calculate_area<T : HasArea>(item : T)  
{  
  println!("Area is : {}",item.area());  
}  
  
fn main()  
{  
  let a = Triangle{base:10.5,height:17.4};  
  let b = Square{side : 4.5};  
  calculate_area(a);  
  calculate_area(b);  
}  