struct Person{
    name: String,
    age: i32,
    favorite_color: String,
}


fn main() {
    let people = vec![
        Person{
            name: "Joseph".to_owned(),
            age:34,
            favorite_color: "Blue".to_owned(),
        },
        Person{
            name: "Bob".to_owned(),
            age:23,
            favorite_color: "Red".to_owned(),
        },
        Person{
            name: "Alexander".to_owned(),
            age:50,
            favorite_color: "Yellow".to_owned(),
        },
        Person{
            name: "Gerald".to_owned(),
            age:12,
            favorite_color: "Purple".to_owned(),
        },
        Person{
            name:String::from("Innocent"),
            age:9,
            favorite_color: "Black".to_owned(),
        },
    ];

     for person in &people{
        if person.age < 26 {
            println!("Name: {}, Age: {}, Favorite Color: {}", person.name, person.age, person.favorite_color);
         }
    }
}