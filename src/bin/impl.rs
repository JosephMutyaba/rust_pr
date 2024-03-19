// struct Temperature{
//     temp_val: f64,
// }

// fn disp_temp(temp: &Temperature){
//     println!("The temperature is: {}", temp.temp_val);
// }


// fn main(){
//     let temp = Temperature{
//         temp_val: 25.0,
//     };
//     disp_temp(&temp);

// }







// struct Temperature{
//     temp_val: f64,
// }


// impl Temperature{
//     fn disp_temp(temp: &Temperature){
//         println!("The temperature is: {}", temp.temp_val);
//     }
// }

// fn main(){
//     let temp = Temperature{
//         temp_val: 25.0,
//     };
//     Temperature::disp_temp(&temp);

// }




struct Temperature{
    temp_val: f64,
}


impl Temperature{
    fn disp_temp(&self){
        println!("The temperature is: {}", self.temp_val);
    }

    //returning Self
    fn hot()->Self{
        Self{
            temp_val: 100.0,
        }
    }


    fn cold()-> Self{
        Self{
            temp_val: 0.0,
        }
    }
}

fn main(){
    let temp = Temperature{
        temp_val: 25.0,
    };
    temp.disp_temp();

    //HOT TEMPERATURE
    let hot = Temperature::hot();
    hot.disp_temp();


    let cold = Temperature::cold();
    cold.disp_temp();

}