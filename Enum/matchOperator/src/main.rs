enum Computerlanguage  
{  
  C,  
  Cplus,  
  Java,  
  Csharp,  
}  
fn language(lang:Computerlanguage)  
{  
 match lang
 {  
   Computerlanguage::C=> println!("C language"),  
   Computerlanguage::Cplus=> println!("C++ language"),  
   Computerlanguage::Java=> println!("Java language"),  
   Computerlanguage::Csharp=> println!("C# language"),  
 }  
}  
fn main()  
{  
 language(Computerlanguage::C);  
 language(Computerlanguage::Cplus);  
 language(Computerlanguage::Java);  
 language(Computerlanguage::Csharp);  
} 