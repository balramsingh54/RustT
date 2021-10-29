

// sliced_value = &data_structure[start_index..end_index]
fn main() {
    let s= "Balram";
    println!("{}",s.len());

    let c= &s[1..4];
    println!("{}",c);


    let arr= [10,20,30,40,50,60,70];
    let cc= &arr[2..5];
    println!("{:?}",cc);

}
