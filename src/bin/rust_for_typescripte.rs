#[derive(Debug, Clone)]
struct Foo { }

fn print_vec(vec: &Vec<Foo>) {
    for v in vec.iter().rev() {
        println!("foo {:?}", v);
    }
}

fn main() {

    let mut values = vec![Foo{}, Foo{}, Foo{}];

    // things happen
    // ...
    let last_foo = values.last();

    // ...
    // mutate the list
    // ...
    values.pop();

    println!("notehu {:?}", last_foo);
}

