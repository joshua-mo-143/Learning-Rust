// NOTES: Vectors are simply resizeable arrays (of a given type). That's it.
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name, age
        }
    }
}
fn main() {
    let mut vec_int: Vec<i32> = vec![5,3,2,4,1];
    let mut vec_float: Vec<f32> = vec![1.1, 1.15, 5.5, 1.32, 2.2];
    
    println!("The int vector currently reads {:?}.", vec_int);
    println!("The float vector currently reads {:?}.", vec_float);

    sort_vec_int(&mut vec_int);
    sort_vec_float(&mut vec_float);

    assert_eq!(vec_int, vec![1,2,3,4,5]);
    println!("The int vector currently reads {:?}.", vec_int);
    println!("The float vector currently reads {:?}.", vec_float);
    sort_vec_structs();

}

fn sort_vec_int(vec_int: &mut Vec<i32>) {
    vec_int.sort();
}

fn sort_vec_float(vec_float: &mut Vec<f32>) {
    vec_float.sort_by(|a, b| a.partial_cmp(b).unwrap());
}

fn sort_vec_structs() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 73),
        Person::new("Josh".to_string(), 25),
        Person::new("Ateemo".to_string(), 26)
    ];

    // sort by first value, which is name here (and therefore by age if there are people with same names)
    people.sort();

    assert_eq!(people, vec![
        Person::new("Ateemo".to_string(), 26),
        Person::new("Josh".to_string(), 25),
        Person::new("Zoe".to_string(), 73)
    ]);

    // sort specifically by a specific value (here we have used age), in ascending order
    people.sort_by(|a, b| a.age.cmp(&b.age));

    assert_eq!(people, vec![
        Person::new("Josh".to_string(), 25),
        Person::new("Ateemo".to_string(), 26),
        Person::new("Zoe".to_string(), 73)
    ])
}