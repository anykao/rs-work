#[macro_use]
extern crate rouille;
extern crate rustc_serialize;

#[derive(RustcEncodable)]
struct MyStruct {
    field1: String,
    field2: i32,
}

fn main() {
    println!("Now listening on localhost:7777");
    rouille::start_server("localhost:7777", move |request| {
        {
            let response = rouille::match_assets(&request, ".");

            if response.is_success() {
                return response;
            }
        }

        router!(request,
        (GET) (/) => {
            rouille::Response::text("hello world")
        },

        (GET) (/mystruct) => {
            rouille::Response::json(&MyStruct { field1: "hello".to_owned(), field2: 5 })
        },

        (GET) (/{name: String}) => {
            rouille::Response::text(format!("hello, {}", name))
        },

         _ => rouille::Response::empty_404()

        )
    })

}
