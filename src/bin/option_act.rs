struct Student{
    name: String,
    locker: Option<i32>,
}


fn main(){
    let students= vec![
        Student{
            name: String::from("Joseph"),
            locker: Some(2),
        },
        Student{
            name: String::from("Bob"),
            locker: Some(1),
        },
        Student{
            name: String::from("Alice"),
            locker: Some(4),
        },
        Student{
            name: String::from("Jane"),
            locker: None,
        },
    ];

    for student in students{
        if student.locker != None{
            println!("Student: {:?}, Locker: {:?}", student.name, student.locker);
        }
    }
}