mod bindings;

use bindings::exports::component::business_logic::handle_data::*;
struct Component;

impl Guest for Component {
    fn handle_data(mut input: MyObject) -> MyObject {
        println!("{:?}", input);

        // Manipulating the object
        input.steps += 1;
        input.processed = Some(true);

        input
    }
}
