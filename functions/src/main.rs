// default function 
fn another_function(){
    println!("this is another function ");
}

// simple parameterized function
fn para_function(x: i32){
    println!("this is parameterized function {}",x);
}

// mixed parameterized function
fn leveled_fun(y:i32, st:&str, chara:char){
    println!("this is y value and this is string value {} {} {}", y, st,  chara);
}

// return type function
fn ret_fun()-> i32 {
    666
}

// parameterized return type function

fn par_retu_fun(n: i32)-> i32 {
   n+1
}

fn main() {

    another_function();
    para_function(5);
    leveled_fun(77, "balram", 'h');
    
    let p= ret_fun();
    println!("this is return type fun value {}",p);

    let q = par_retu_fun( 900);
    println!("the value of q is {}",q);
}
