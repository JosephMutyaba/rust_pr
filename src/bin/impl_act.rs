enum Color{
    Red,
    Green,
    Blue,
}

impl Color {
    fn print(&self){
        match self {
            Color::Red => println!("Color: Red"),
            Color::Green => println!("Color: Green"),
            Color::Blue => println!("Color: Blue"),
        }
    }
}

struct Dimensions {
    length : i32,
    width : i32,
    height : i32,
}


impl Dimensions {

    fn new(length : i32, width : i32, height : i32)->Self{
        Self{
            length,
            width,
            height,
        }
    }

    fn print(&self){
        println!("Length: {}", self.length);
        println!("Width: {}", self.width);
        println!("Height: {}", self.height);
    }
}

struct Box{
    weight : i32,
    color : Color,
    dimensions:Dimensions,
}

impl Box{

    fn new(weight : i32, color : Color , dimensions:Dimensions)->Self{
        Self{
            weight,
            color,
            dimensions,
        }
    }

    fn print_characteristics(&self){
        println!("Weight: {}", self.weight);
        self.color.print();
        self.dimensions.print();
    }
}

fn main(){


    // let d1= Dimensions{
    //     length : 10,
    //     width : 20,
    //     height : 30,
    // };
    let d1 = Dimensions::new(20,34,56);

    let b1 = Box::new(10, Color::Red, d1);
    b1.print_characteristics();
}