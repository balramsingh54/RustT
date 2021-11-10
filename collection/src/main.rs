//Let v : Vec<i32> = Vec::new();  
// A vector can be created by using Vec::new() function. Let's look at this:

// let v = vec![10,20,30,40,50];    this is another way to create a vector

//let v = vec![2 ; i];  
//In the above declaration, vector 'v' is created using vector macro which 
//contains the value 2 'i' times.



fn value (n:Option<&i32>){
    match n {
        Some(n)=> println!("fourth element of vector is :{}",n),
        none => println!("none"),
    }
}

fn main() {
    let v= vec![1,2,3,4,5,6,7,8,9];
    println!("this is first value of vector :{}", v[0]);
    println!("this is second value of vector :{}", v[1]);
    println!("this is third value of vector :{}", v[2]);
    println!("this is fourth value of vector :{}", v[3]);
    println!("this is fifth value of vector :{}", v[4]);
    println!("this is six value of vector :{}", v[5]);
    println!("this is seven value of vector :{}", v[6]);
    println!("this is eithth value of vector :{}", v[7]);
    println!("this is ninth value of vector :{}", v[8]);

    let a:Option<&i32> = v.get(3);
    let b:Option<&i32> = v.get(13);

    value(a);
    value(b);

    // iterating over the vector 
    for i in v{
        println!("{}",i);
    }


    let mut vv= Vec::new();
    vv.push('B');
    vv.push('a');
    vv.push('l');
    vv.push('r');
    vv.push('a');
    vv.push('m');
    for i in vv{
        print!("{}",i);
    }

}
