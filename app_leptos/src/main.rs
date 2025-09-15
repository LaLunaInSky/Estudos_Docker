use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn main() {
    mount_to_body(
        || view! {
            <h1>
                "Hello, World"
            </h1>
        }
    );
}
