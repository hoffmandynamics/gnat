// Copyright (c) 2016 The Rouille developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

extern crate rouille;

use rouille::Response;

fn main() {
    println!("Now listening on 0.0.0.0:64000");

    rouille::start_server("0.0.0.0:64000", move |request| {
        {
            let response = rouille::match_assets(&request, ".");
            if response.is_success() {
                return response;
            }
        }
        Response::html(
            "404 error. Try <a href=\"/README.md\"`>README.md</a> or \
                        <a href=\"/src/lib.rs\">src/lib.rs</a> for example.",
        )
        .with_status_code(404)
    });
}
