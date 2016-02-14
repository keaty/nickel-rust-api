// Tutorial by Auth0: Build an API in Rust with JWT Authentication using Nickel.rs
// Learn how to implement a simple REST API with JWT Authentication in Rust using the Nickel.rs web framework and the MongoDB Rust Driver.
// https://auth0.com/blog/2015/11/30/build-an-api-in-rust-with-jwt-authentication-using-nickelrs/

// src/main.rs

#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/users", middleware! { |_, response|

        format!("Hello from GET /users")

    });

    router.post("/users/new", middleware! { |_, response|

        format!("Hello from POST /users/new")

    });

    router.delete("/users/:id", middleware! { |_, response|

        format!("Hello from DELETE /users/:id")

    });

    server.utilize(router);

    server.listen("127.0.0.1:9000");
}
