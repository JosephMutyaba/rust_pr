//derive key word
#[derive(Debug, Clone, Copy)]
enum Salaries {
    Copper, Diamond,
}

#[derive(Debug, Clone, Copy)]
struct WorkType{
    //profession: String,
    salary: Salaries,
}


fn print_emp(workType: WorkType){
    println!("{:?}", workType);
}

fn main(){
    let work_type = WorkType{
        //profession: String::from("Developer"),
        salary: Salaries::Copper,
    };
    // println!("Profession: {:?}", work_type);
    print_emp(work_type);
    print_emp(work_type);
}