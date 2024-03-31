use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);
    list.push_back(8);
    list.push_back(9);
    list.push_back(10);

    println!("List contains:");
    for i in list.iter() {
        println!("{}", i);
    }

    list.pop_front();

    println!("List contains after pop_front:");

    for i in list.iter() {
        println!("{}", i);
    }

    list.pop_back();

    println!("List contains after pop_back:");

    for i in list.iter() {
        println!("{}", i);
    }

    list.remove(|&e| e == 3);

    println!("List contains after remove:");

    for i in list.iter() {
        println!("{}", i);
    }

}
