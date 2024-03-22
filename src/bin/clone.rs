#[derive(Clone, Copy, Debug)]
enum Colors{
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Clone, Copy, Debug)]
struct Car{
    color: Colors,
    wheels: i32,
}

fn printCar(car: &Car){
    println!("Car color: {:?}", car.color);
    println!("Car wheels: {:?}", car.wheels);
}

fn main() {
    let carColor = Colors::Red;
    let car1 = Car{color: carColor, wheels: 4};

    printCar(&car1);
    printCar(&car1);

}