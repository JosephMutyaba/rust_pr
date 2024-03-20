struct Channel {
    name: String,
}



fn main(){
    let  channels = vec![
        Channel{
            name: "HBO".to_owned(),
        },
        Channel{
            name: String::from("CNN"),
        },
        Channel{
            name: String::from("BBC"),
        },
    ];

    for channel in channels{
        println!("Channel: {}", channel.name);
    }
}