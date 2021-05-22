mod to_do;
use to_do::structs::traits::create::Create;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!("it's done with title {}", item.super_struct.title),
    }
}
