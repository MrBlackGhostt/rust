use std::future;

fn main() {
    struct Person {
        name: String,
        age: u32,
        email: String,
    }

    let p1 = Person {
        name: String::from("Hemant"),
        age: 26,
        email: String::from("hemantKumar4213@gmail.com"),
    };

    let pn = Person {
        name: String::from("value"),
        age: 20,
        email: String::from("Test@test.com"),
    };
    print!(
        "
        Name: {}
        age: {},
        email: {}",
        p1.name, p1.age, p1.email
    )
}
