fn area(width:i32, height:i32) -> i32{
    width*height
}

//factoring with tupple 

fn aree(rectangle:(i32, i32)) -> i32{
    rectangle.0*rectangle.1
}

// factoring with struct

struct Triangle{
    width:f32,
    height:f32,
}

fn areaaa(t:&Triangle)-> f32{
    0.5*t.width*t.height
}

fn main() {
    let a= area(5, 10);
    println!("area is {}", a);

    let rect1 = (3,4);
    println!("area of rectanglel is {}",aree(rect1));

    let triangl = Triangle{
        width:40.0,
        height:50.0
    };

    println!("Area of the triangle is {}",areaaa(&triangl));

}
