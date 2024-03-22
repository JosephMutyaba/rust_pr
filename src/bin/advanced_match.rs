enum Things{
    Percentage(f64),
    ColorStrength(i32),
    Temperature(f64),
    name(String),
}

enum Puzzle{
    Number(i32),
    Color(String),
    Something(Things),

}

struct User{
    name: String,
    age: i32,
    favorite_color: String,
}

fn main(){
    let  thing = Things::Percentage(45.9);

    match thing{
        Things::Percentage(x) => println!("Percentage: {}", x),
        Things::ColorStrength(x) => println!("Color Strength: {}", x),
        Things::Temperature(x) => println!("Temperature: {}", x),
        Things::name(x) => println!("Name: {}", x),
        other => println!("This is"),
    }

    let user:User = User{
        name: "<Bob>".to_owned(),
        age: 25,
        favorite_color: String::from("Blue"),
    };


    match user {
        User{name, age:30,favorite_color} => println!("Age: {}", name),
        User{favorite_color,..} => println!("Favorite Color: {}", favorite_color),
        other => println!("This is"),
    }
}