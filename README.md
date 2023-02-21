# 🌸 Blossom

<!-- markdownlint-disable -->
<div align="center">
  <strong>Secure:</strong> blossom.sombia.com:5443
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <strong>Unsecure:</strong> blossom.sombia.com:5080
</div>

<br>

<div align="center">
    <img src="assets/example.png" alt="terminal screenshot showing off styled output">
</div>
<!-- markdownlint-enable -->

-----

Blossom is a MUD (Multi-User Dungeon) game engine written in Rust.

This is still an early work-in-progress and there will be sweeping, breaking
changes often as I refine the architecture and API. If you actually want to
build a MUD, you'd be best off using an existing codebase. Any of the old
C-bases, Evennia *(Python)*, or Ranvier *(JavaScript)* are excellent options.

## Live Game

The test server listed at the top of the README should be online most of the
time, though the database may be reset at any time.

Connecting via Telnet: `telnet blossom.sombia.com 5080`

*In order to connect to the secure port, you must use a MUD client which
supports TLS encryption. I recommend [Mudlet](https://www.mudlet.org/) for a
cross-platform option!*

## Contributing

Prerequesites:

- Rust Nightly 1.69
- PostgreSQL 15

In addition, these are recommended, **optional** dev dependencies:

- Python 3.10 *(tooling)*
- SQLx CLI *(migrations)*
- NodeJS 19 *(dashboard / content creation tools / web client)*

Instructions to build:

1. `git clone https://github.com/robertwayne/blossom`
2. Modify the `.env.TEMPLATE` file in the root directory with your local
   Postgres details. *(This is neccesary for SQLx to do compile-time SQL
   validation).*
3. *(requires SQLx CLI)* From the root directory, run `sqlx migrate run` to
   apply the schema.
4. Hack away!
5. *(requires SQLx CLI & Node)* Run `./check.sh` before you open a PR. If you do
   not have the optional dependencies, run the commands manually.

*By contributing, you agree that any code submitted by you shall be
dual-licensed under MIT and Apache-2.0.*

## License

Blossom source code is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE)**

at your option.
