

enum Ticket{
    BackStage(String, f64),
    Vip(String, f64),
    Standard(f64),
}



fn main() {
    let  ticket_vector = vec![
        Ticket::BackStage("Bob".to_owned(), 10.0),
        Ticket::Vip("Alexander".to_owned(), 10.0),
        Ticket::Standard(10.0),
    ];

    for ticket in ticket_vector{
        match ticket{
            Ticket::BackStage(a, x) => println!("BackStage:name->{}, price->{}",a, x),
            Ticket::Vip(a, x) => println!("Vip:name->{}, price->{}",a, x),
            Ticket::Standard(x) => println!("Standard: price->{}", x),

        }
    }

}