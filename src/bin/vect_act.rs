fn main(){
    let numbers = vec![10, 20,30,40, 56];

    for  i in &numbers{
        match i {
            30 => println!("thirty"),
            _ => println!("{}", i),
        }
    }

    println!("Total elements: {}", numbers.len());


}