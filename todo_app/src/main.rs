mod to_do;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => println!("it's pending with title {}", item.super_struct.title),
        ItemTypes::Done(item) => println!("it's pending with title {}", item.super_struct.title),
    }
}
