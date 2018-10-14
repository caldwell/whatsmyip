What's My IP?
=============

Ever find yourself wondering that? Then set up this tiny HTTP handler on a
remote server somewhere and then hit it with your browser or curl.

## Building

Requires [Rust](https://www.rust-lang.org/).

1. cargo build --release
2. ./target/release/whatsmyipd

## Configuring

Set the `WHATSMYIP_PORT` env var to configure the listener port. The default
is `3141`.

## License

Your choice: MIT, GPL (any version), MPL, Artistic. It's too short for me to care about.

Copyright © 2012 David Caldwell <david@porkrind.org>
