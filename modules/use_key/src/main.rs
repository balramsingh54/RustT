#[derive(Debug)]
enum Flagcolor  
{  
 Orange,  
 White,  
 Green,  
}  
use Flagcolor::{Orange,White,Green};  
fn main()  
{  
  let o= Orange;  
  let _w= White;  
 let _g= Green;  
 println!("{:?}",o);  
println!("{:?}",_w);  
println!("{:?}",_g);  
}  