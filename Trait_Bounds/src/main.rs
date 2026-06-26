fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

struct NotCloneable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    // duplicate(NotCloneable); Compiler error[E0277]: the trait bound `NotCloneable: Clone` is not satisfied
    println!("{pair:?}");
}