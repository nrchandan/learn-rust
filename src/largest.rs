fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let str_list = vec!["hello", "help", "hi"];
    let result = largest(&str_list);
    println!("The largest string is {}", result);

    let str_list = vec![String::from("hello"), String::from("help"), String::from("hi")];
    let result = largest(&str_list);
    println!("The largest string is {}", result);
}
