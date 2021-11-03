mod a{
    pub fn a(){
        println!("this is a");
    }


    pub mod b{
        pub fn b(){
            println!("this is b");
        }
    }
}

fn main() {
    a::a();
    a::b::b();
    
}
