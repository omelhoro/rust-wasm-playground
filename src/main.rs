extern crate rust_wasm_playground;
extern crate yew;

use rust_wasm_playground::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
