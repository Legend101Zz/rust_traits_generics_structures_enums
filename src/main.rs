 struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age : u8,
    sex: char,
    country : String,
}

trait GeneralInfo {
    fn info(&self)->(&str,u8,char);
    fn country_info(&self)-> &str ;
}

// impl Person {

// fn new() -> Self {
//     Person {
//         name: String::from("Mrigesh Thakur"),
//         citizenship: String::from("India"),
//         age: 20,
//         gender: 'M',
//         salary: 20_000,
//     }

// }


//     fn compute_taxes(&self) -> f32 {
//         (self.salary as f32 / 3.) * 0.5
//     }
// }

// fn main() {
//  let person1 = Person::new();

//    let x = 2;

//    let x = x +1 ;  

//  let y = vec![1,2];
       
//     println! {"{} tax is {}",person1.name, person1.compute_taxes()}
// }

impl GeneralInfo for Person {
fn info (&self)->(&str, u8,char){
(&self.name , self.age, self.gender )
}

fn country_info(&self)-> &str{
    &self.citizenship
}

}


fn main(){


    let person1 : Person =    Person {
                 name: String::from("Mrigesh Thakur"),
          citizenship: String::from("India"),
                 age: 20,
                 gender: 'M',
                 salary: 20_000,
             };

             println!("{:?}", person1.info())
}


// added notes