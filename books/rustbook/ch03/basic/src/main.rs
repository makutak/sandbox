mod my_box;
mod my_iterator;
mod my_loop;
mod my_macro;
mod result;

fn main() {
    println!("############# Result #################");
    result::exec();
    println!("\n");

    println!("############## Box ##############");
    my_box::exec();
    println!("\n");

    println!("############# loop label ########");
    my_loop::exec();
    println!("\n");

    println!("############# Iterator ########");
    my_iterator::exec();
    println!("\n");

    println!("############# Macro ########");
    my_macro::output_macro();
    println!("\n");
    my_macro::access_resource_macro();
    println!("\n");
    my_macro::support_impliment_macro();
    println!("\n");
}
