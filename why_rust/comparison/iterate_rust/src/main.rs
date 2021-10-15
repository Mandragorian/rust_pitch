fn main() {
    let mut list = vec![1,2,3,4];

    for elem in &list {
        list.push(elem);
    }
}
