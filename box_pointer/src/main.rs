mod tree;

fn main() {
//    let vec_to_sort = vec![24, 0, 23, -1, 34, 1, 48, 3, 20]; 
    let vec_to_sort = vec!['a', 'b', 'd', 'x', 'c', 'z', 'f', 'g'];
    let mut head = tree::Node::new(vec_to_sort[0]);
    for element in vec_to_sort.iter() {
        head.add(*element)
    }
    println!("{:#?}", head);
}
