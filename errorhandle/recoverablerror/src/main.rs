use std::fs::File;  
fn main()   
{  
    // let f:u32 = File::open("vector.txt");  
    //The difference between the expect() and unwrap() method is that the error
    // message is passed as a parameter to the expect() method
    // while unwrap() method does not contain any parameter.
}


use std::fs::File;  
fn main()  
{  
   let f = File::open("vector.txt");  
   match f   
   {  
       Ok(file) => file,  
       Err(error) => {  
       panic!("There was a problem opening the file: {:?}", error)  
     },  


File::open("hello.txt").expect("Not able to find the file hello.txt");  

   }


use std::io;  
use std::io::Read;  
use std::fs::File;  
fn main()  
{  
  let a = read_username_from_file();  
  print!("{:?}",a);  
}  
fn read_username_from_file() -> Result<String, io::Error>   
{  
    let mut s = String::new();  
   File::open("a.txt")?.read_to_string(&mut s)?;  
   Ok(s)  
} 



use std::fs::File;
fn main() {
   let f = File::open("main.jpg");   // main.jpg doesn't exist
   match f {
      Ok(f)=> {
         println!("file found {:?}",f);
      },
      Err(e)=> {
         println!("file not found \n{:?}",e);   //handled error
      }
   }
   println!("end of main");
}