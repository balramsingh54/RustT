// outer module and inside pub fn and pri fn innder module pub fn and fn 

mod outer{
    pub fn a(){
        println!("function a ");
    }

    pub fn b(){
        println!("function b");
    }

     mod inner {
        pub fn c(){
            println!("function c");
        }

        pub fn d(){
            println!("function d");
        }
    }
}

fn main() {
    outer::a();
    outer::b();
    outer::inner::c();
    outer::inner::d();
}
