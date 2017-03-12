Community Garden Helper (CoG)
==============================

Built using Rust with Iron and Diesel.

More details forthcoming once it actually does something.

Licenced as AGPL.

Usage
=====

Pre-requisites:

* It is probably best to run this on a reasonably modern Linux distro
* Rust and Cargo must be installed on your system (see: https://doc.rust-lang.org/book/nightly-rust.html)

Install Diesel CLI:

    cargo install diesel_cli

Create an environment file indicating the location of your database:

    echo "DATABASE_URL=postgres://username:password@hostname/database" > .env

Apply schema migrations using Diesel (this requires that `$HOME/.cargo/bin` is in your executable path):

    diesel migration run

Run the application using Cargo:

    cargo run
