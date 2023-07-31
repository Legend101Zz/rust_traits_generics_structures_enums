struct Person {
citizenship : String ,
name: String,
age : i32,
gender: char ,
salary : i32

} 

fn main() {

let person1: Person = Person {
   name : String::from("Mrigesh Thakur") ,
   citizenship : String::from("India"),
   age: 20,
   gender : 'M',
   salary : 20_000
};

println!{"{}",person1.name}

}
