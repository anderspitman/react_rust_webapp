#[macro_use]
extern crate rouille;

use rouille::Response;

fn main() {
    let index = include_str!("../ui/dist/index.html");
    let bundle = include_str!("../ui/dist/bundle.js");
    rouille::start_server("0.0.0.0:5000", move |request| {

        let response = router!(request,
            (GET) ["/"] => {
                Response::html(index)
            },
            (GET) ["/bundle.js"] => {
                Response::text(bundle)
            },
            _ => {
                Response::empty_404()
            }
        );

        response
    });
}
