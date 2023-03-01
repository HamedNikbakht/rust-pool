struct NeededItems {
    quantity: i32,
    id: i32,
}

fn show_quantity(item: &NeededItems) {
    println!("quantity: {:?}", item.quantity);
}

fn show_id(item: &NeededItems) {
    println!("id: {:?}", item.id);
}

fn main() {
    let my_item = NeededItems {
        quantity: 3;
        id: 99;
    };
    show_quantity(&my_item);
    show_id(&my_item);
}

