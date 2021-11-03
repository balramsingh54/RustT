mod a{
    pub fn single_function(){
        println!("this is first module ");
    }
}


mod C{
    pub fn C(){
        println!("this is c lagnguage");
    }
}

mod cpp{
    pub fn cpp(){
        println!("this is object oriented language ");
    }
}

fn main() {
    a::single_function();

    C::C();
    cpp::cpp();
}
