# 🌸 Blossom

<!-- markdownlint-disable -->
<div>
  <strong>Secure:</strong> blossom.sombia.com:5443
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <strong>Unsecure:</strong> blossom.sombia.com:5080
</div>
<!-- markdownlint-enable -->

-----

Blossom is an opinionated MUD game engine written in Rust.

This is still a VERY early work-in-progress and there will be sweeping,
breaking changes constantly as I refine the architecture and API. If you
actually want to build a MUD, you'd be best off using an existing codebase. Any
of the old C-bases, Evennia *(Python)*, or Ranvier *(JavaScript)* are excellent
options.

## Live Game

The test server listed at the top of the README should be online most of the
time. Be warned that the database may be reset at any time.

Connecting via Telnet: `telnet blossom.sombia.com 5080`

*In order to connect to the secure port, you must use a MUD client which support
TLS encryption. I recommend [Mudlet](https://www.mudlet.org/) for a
cross-platform option!*

## Usage

```rs
// main.rs
use blossom::prelude::*;

fn main() -> Result<(), Box dyn std::error::Error>> {
    let server = Server::new();
    let world = World::new();

    server.listen(world)?;

    Ok(())
}
```

## Contributing

Prerequesites:

- Rust Nightly 1.68
- PostgreSQL 14

In addition, these are recommended, **optional** dev dependencies:

- Python 3.10 *(tooling)*
- SQLx CLI *(migrations)*
- Node 19 *(dashboard / content creation tools / web client)*

Instructions to build:

1. `git clone https://github.com/robertwayne/blossom`
2. Modify the `.env.TEMPLATE` file in the root directory with your local
   Postgres details. *(This is neccesary for SQLx to do compile-time SQL
   validation).*
3. *(requires SQLx CLI)* From the root directory, run `sqlx migrate run` to
   apply the schema.
4. Hack away!
5. Run `./check.sh` before you open a PR.

*By contributing, you agree that any code submitted by you shall be
dual-licensed under MIT and Apache-2.0.*

## License

Blossom source code is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE)**

at your option.
