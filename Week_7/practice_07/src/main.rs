fn main() {
    
    let a1:[i32;4] = [10,20,30,40];
    println!("\n array with datatype");
    println!("array {:?}",a1 );
    println!("array size is :{}",a1.len());


    let a2 =[10.4,20.7,30.4,40.9,51.2,72.2];
    println!("\n without datatype");
    println!(" array is {:?}", a2);
    println!("array size is :{}",a2.len() );


    let a3:[i32;8] = [-1;8];
    println!("\n array with deffault values");
    println!("array is {:?}",a3 );
    println!("array size is {}", a3.len());
}
