// struct Book{
//     pages: i32,
//     rating: i32,
// }

// fn display_pages(book: &Book){
//     println!("Pages: {}", book.pages);
// }

// fn display_rating(book: &Book){
//     println!("Rating: {}", book.rating);
// }

// fn main(){
//     let book = Book{
//         pages: 100,
//         rating: 4,
//     };


//     display_pages(&book);
//     display_rating(&book);
// }



struct Item{
    id: i32,
    qty: i32,
}

fn display_id(item: &Item){
    println!("ID: {}", item.id);
}

fn display_qty(item: &Item){
    println!("Qty: {}", item.qty);
}

fn main(){
    let item = Item{
        id:23,
        qty: 12,
    };

    display_id(&item);
    display_qty(&item);
}

