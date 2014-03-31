#[crate_id = "contamers-demo#0.1.0"];
#[crate_type = "bin"];
#[license = "MIT"];

extern crate contamers;

use contamers::list;

#[deriving(Show)]
enum Gender {
    Male,
    Female,
    Porpoise
}

#[deriving(Show)]
struct Person {
    gender: Option<Gender>,
    name: ~str
}

fn with_angry_name(person: &Person) -> Person {
    match *person {
        Person { ref gender, ref name } => {
            let new_name = name.to_ascii().to_upper().as_str_ascii().to_owned();
            Person { gender: *gender, name: new_name }
        }
    }
}

pub fn main() {
    use list::List;

    let tom = Person { gender: Some(Male), name: ~"Tom" };
    let joe = Person { gender: Some(Porpoise), name: ~"Joe the Porpoise" };
    let ann = Person { gender: Some(Female), name: ~"Ann Veal" };

    let a = [tom, joe, ann];
    let persons: List<Person> = a.iter().map(with_angry_name).collect();

    println!("the people:");
    for person in persons.iter() {
        println!("{}", person);
    }
}
