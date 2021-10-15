fn main() {
    let mut list = vec![1,2,3,4];

    let mut i = 0;
    while i < list.len() {
        println!("{}", list[i]);
        list.push(list[i]);
        i += 1;
    }
}
