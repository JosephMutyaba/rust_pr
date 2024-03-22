struct Survey{
    qn1: Option<String>,
    qn2: Option<bool>,
}

fn main() {
    let survey1 = Survey{
        qn1: Some("What is your name?".to_owned()),
        qn2: Some(true),
    };
    match survey1.qn2{
        Some(qn) => println!("Question 1: {}", qn),
        None => println!("Question 1: None"),
    }



    let survey2 = Survey{
        qn1: None,
        qn2: Some(true),
    };

    match survey2.qn1{
        Some(_qn) => println!("Question 2: {:?}", survey2.qn2),
        None => println!("Question 2: None"),
    }
}