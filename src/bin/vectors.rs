struct Test{
    score: i32,
}

fn main(){
    let vector1= vec![
        Test{
            score: 100,
        },
        Test{
            score: 90,
        },
        Test{
            score: 80,
        },

    ];


    //for in loop
    for vector in vector1{
        println!("The score is: {}", vector.score);
    }

    println!();

    let mut vector2= Vec::new();

    vector2.push(Test{score:10});
    vector2.push(Test{score:20});
    vector2.push(Test{score:30});

    for vector in vector2{
        println!("The score is: {}", vector.score);
    }
}








