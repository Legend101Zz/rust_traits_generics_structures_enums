struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}

fn main() {
    let person1: Person = Person {
        name: String::from("Mrigesh Thakur"),
        citizenship: String::from("India"),
        age: 20,
        gender: 'M',
        salary: 20_000,
    };

    println! {"{} tax is {}",person1.name, person1.compute_taxes()}
}
