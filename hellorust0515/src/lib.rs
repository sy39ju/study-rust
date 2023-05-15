enum LL {
    None,
    Some(Box<LLPerson>),
}

pub struct LLPerson {
    name: String,
    age: u32,
    next: LL,
}

impl LL {
    fn add(&mut self, person: Box<LLPerson>) {
        let mut node = if let LL::Some(node) = self {
            node
        } else {
            return;
        };
        loop {
            match node.next {
                LL::Some(ref mut next) => {
                    node = next;
                }
                LL::None => {
                    node.next = LL::Some(person);
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Node {
    data: Person,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: Person) -> Self {
        Node { data, next: None }
    }
}

use std::fmt::{Display, Formatter};
impl Display for LinkedList {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut node = if let Some(ref node) = self.head {
            node
        } else {
            return Ok(());
        };
        loop {
            match node.next {
                Some(ref next) => {
                    write!(f, "{:?}", next.data);
                    node = next;
                }
                None => break,
            }
        }
        Ok(())
    }
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: Some(Box::new(Node {
                data: Person::new("head", 0, 0),
                next: None,
            })),
        }
    }

    pub fn add(&mut self, data: Person) {
        let mut node = if let Some(ref mut node) = self.head {
            node
        } else {
            return;
        };
        loop {
            match node.next {
                Some(ref mut next) => node = next,
                None => {
                    node.next = Some(Box::new(Node::new(data)));
                    break;
                }
            }
        }
    }

    pub fn convert(&mut self) -> Vec<Person> {
        let mut list_vector = Vec::new();
        let mut node = if let Some(ref node) = self.head {
            node
        } else {
            return list_vector;
        };

        loop {
            match node.next {
                Some(ref next) => {
                    list_vector.push(Person {
                        name: next.data.name.clone(),
                        height: next.data.height,
                        age: next.data.age,
                    });
                    node = next;
                }
                None => break,
            }
        }

        return list_vector;
    }

    pub fn print(&self) {
        let mut node = if let Some(ref node) = self.head {
            node
        } else {
            return;
        };
        loop {
            match node.next {
                Some(ref next) => {
                    println!("data: {:?}", next.data);
                    node = next;
                }
                None => break,
            }
        }
    }
    pub fn remove(&mut self, data: Person) {
        let mut node = if let Some(ref mut node) = self.head {
            node
        } else {
            return;
        };
        loop {
            match node.next {
                Some(ref next) => {
                    if next.data.name == data.name {
                        let node2 = node.next.as_mut().unwrap();
                        node.next = node2.next.take();
                        break;
                    }
                    node = node.next.as_mut().unwrap();
                }
                None => break,
            }
        }
    }
}

pub fn echo_num(num: u32) {
    println!("num: {}", num.to_string());
}

pub fn echo_num_multi(num: u32, mut times: i32) {
    if times > 10 {
        times = 10;
    }
    for _ in 0..times {
        echo_num(num);
    }
}

pub fn echo_str_ntimes(to_say: &String, times: i32) {
    for _ in 0..times {
        echo_str(to_say);
    }
    let a = "hi";
    println!("a: {}", a);

    let b: Box<i32> = Box::new(10);
    println!("b: {}", b);
}

pub fn echo_str(to_say: &String) {
    println!("say: {}", to_say);
}

#[derive(PartialEq)]
pub struct Person {
    name: String,
    height: u32,
    age: u32,
}

use std::fmt::Debug;
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "My Person {{ name: {}, height: {}, age: {} }}",
            self.name, self.height, self.age
        )
    }
}

impl Person {
    pub fn new(name: &str, height: u32, age: u32) -> Person {
        Person {
            name: name.to_string(),
            height,
            age,
        }
    }
}

pub fn struct_exam() {
    let person = Person::new("Ju", 180, 41);
    println!("person: {:?}", person);
}

pub fn test() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    println!("r: {}", r);
    r = &y;
    println!("r: {}", r);
    println!("r: {}", x);
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;

    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere",
    }
}

#[derive(Debug)]
struct Account {
    name: String,
    language: String,
    age: u32,
}

fn match_account() {
    let account = {
        Account {
            name: "Ju".to_string(),
            language: "Rust".to_string(),
            age: 41,
        }
    };

    match account {
        Account {
            ref name,
            ref language,
            ..
        } => {
            println!("{}: {}", name, language);
            println!("{:?}", account);
        }
    }
}

pub fn expr() -> i32 {
    let status = if true { 200 } else { 500 };
    if let 200 = status {
        println!("OK");
    } else {
        println!("NG");
    }
    let x: Option<i32> = None;
    if let Some(x) = x {
        println!("OK {x}");
    } else {
        println!("NOK {x:?}");
    }

    while let Some(x) = x {
        println!("OK {x:?}");
    }
    for i in 0..20 {
        println!("{}", i);
    }
    status
}

pub fn write_test() -> Result<(), std::io::Error> {
    use std::io::Write;

    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello")?;
    Ok(())
}

pub fn test2() {
    use std::io::Write;

    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;
    writer.write_all(b"hello").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_num() {
        echo_num(10);
    }

    #[test]
    fn test_echo_num_multi() {
        echo_num_multi(10, 10);
    }

    #[test]
    fn test_echo_str() {
        echo_str(&"hello".to_string());
    }

    #[test]
    fn test_echo_str_ntimes() {
        echo_str_ntimes(&"hello".to_string(), 10);
    }

    #[test]
    fn test_person() {
        let person = Person::new("Ju", 180, 41);
        assert_eq!(person.name, "Ju");
        assert_eq!(person.height, 180);
        assert_eq!(person.age, 41);
    }

    #[test]
    fn test_option() {
        let x = Some(10);
        let y = Some(20);
        let z = x.or(y);
        let k = None;
        let l = k.or(y);
        assert_eq!(z, x);
        assert_eq!(k, None);
        assert_eq!(l, y);
    }

    #[test]
    fn test_describe_point() {
        let result = describe_point(0, 0);
        assert_eq!(result, "at the origin");
    }

    #[test]
    fn test_match_account() {
        match_account();
    }

    #[test]
    fn test_expr() {
        let result = expr();
        assert_eq!(result, 200);
        assert_eq!(result, 200);
    }

    #[test]
    fn test_ll_add() {
        let mut ll = LinkedList::new();
        ll.add(Person {
            name: "Ju".to_string(),
            height: 180,
            age: 41,
        });
        ll.add(Person {
            name: "Lee".to_string(),
            height: 180,
            age: 41,
        });
        ll.add(Person {
            name: "Kim".to_string(),
            height: 180,
            age: 41,
        });
        let vector: Vec<Person> = ll.convert();
        assert_eq!(
            vector,
            vec![
                Person {
                    name: "Ju".to_string(),
                    height: 180,
                    age: 41
                },
                Person {
                    name: "Lee".to_string(),
                    height: 180,
                    age: 41
                },
                Person {
                    name: "Kim".to_string(),
                    height: 180,
                    age: 41
                }
            ]
        );
    }

    #[test]
    fn test_ll_remove() {
        let mut ll = LinkedList::new();
        ll.add(Person {
            name: "Ju".to_string(),
            height: 180,
            age: 41,
        });
        ll.add(Person {
            name: "Lee".to_string(),
            height: 180,
            age: 41,
        });
        ll.add(Person {
            name: "Kim".to_string(),
            height: 180,
            age: 41,
        });
        ll.add(Person {
            name: "Ha".to_string(),
            height: 180,
            age: 41,
        });
        ll.remove(Person {
            name: ("Lee".to_string()),
            height: (180),
            age: (41),
        });
        let vector: Vec<Person> = ll.convert();
        assert_eq!(
            vector,
            vec![
                Person {
                    name: "Ju".to_string(),
                    height: 180,
                    age: 41
                },
                Person {
                    name: "Kim".to_string(),
                    height: 180,
                    age: 41
                },
                Person {
                    name: "Ha".to_string(),
                    height: 180,
                    age: 41
                }
            ]
        );
    }
}
