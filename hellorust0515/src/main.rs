use hellorust0515::*;

fn main() {
    let num: u32 = 123_u32;
    echo_num(num);
    echo_num_multi(num, 100);

    let say = "hello".to_string();
    echo_str(&say);
    echo_str_ntimes(&say, 10);

    test();
    struct_exam();

    expr();
    write_test();

    let mut persons = LinkedList::new();
    let person = Person::new("Ju", 180, 41);
    let person2 = Person::new("Ju2", 180, 41);
    persons.add(person);
    persons.add(person2);
    println!("persons: {:?}", persons);
}
