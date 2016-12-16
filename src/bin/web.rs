#[macro_use]
extern crate rouille;

fn main() {
    rouille::start_server("localhost:8000", move |request| {
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

        (GET) (/{name: String}) => {
            rouille::Response::text(format!("hello, {}", name))
        },

         _ => rouille::Response::empty_404()

        )

    })

}
