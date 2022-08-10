# 🌸 Blossom

Blossom is an opinionated MUD engine written in Rust.

**This is still a VERY early work-in-progress and there will be sweeping,
breaking changes constantly as I refine the architecture and API.** If you
actually want to build a MUD, you'd be best off using an existing codebase. Any
of the old C-bases, Evennia *(Python)*, or Ranvier *(JavaScript)* are excellent
options.

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

*This is a minimal working example; it has no game content. If you want to see
the code and game data for Coven, the example game above, it is also
**[open-source](https://github.com/robertwayne/coven)**.*

## Contributing

Prerequesites:

- Rust 1.65 (nightly; Blossom uses some nightly-only features)
- PostgreSQL 14
- *(optional)* Python 3 (tooling)
- *(optional)* SQLx CLI (migrations)
- *(optional)* Node 18 (dashboard / content creation tools)

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
