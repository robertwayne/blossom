# Architecture

This is a brief document describing the current architecture of certain elements
within the engine.

*This is not complete, and will be ever-evolving as the architecture stabilizes.*

## Project Structure

Blossom is broken up into various smaller crates to enscapulate various
functionality. Ideally, these crates are reusable outside of Blossom - but that
is not a hard requirement.

### blossom_ansi

Moved into a stand-alone crate: **[Iridescent](https://github.com/robertwayne/iridescent)**.

### blossom_telnet

Partial telnet implementation. Focuses on parts needed for general MUD
communication, and will eventually include MUD protocols such as GMCP.

### blossom_dynamic

Used for enabling the `dylib` crate feature for dynamic linking.

### blossom_internal

Glue module which exposes the public API of all the crates under one name.

### blossom_core

Contains all core functionality, like connection handlers, the game loop,
message handling, database querying, systems, etc. This module will often be
broken up into new crates as they outgrow the core.

This is really just the 'hatchery' for new modules.

### blossom_web

Web server which runs in the background as a management dashboard,
log aggregator, analytics, and access to the content creation tools.

## Channels and Messaging

The engine is 'split' into three major pieces: the server, which manages
connections and incoming/outgoing packets. This is async and multithreaded
thanks to Tokio.

The game loop runs on its own blocking thread, and is NOT async - though it can
spawn its own threads for tasks outside the normal loop.

These two systems CANNOT directly interact with each other, and instead talk
through a broker which passes GameEvent and ClientEvent messages between them,
but also handles intermediary processes, like overseeing the entire connection
pool and mapping in-game player IDs to their TCP streams.

## Game Loop

The game loop iterates at a fixed tick rate (configurable) in order.

**Systems** are structs that implement the `System / SystemReadOnly` trait, and
are a way to inject into the game loop. They are run on every tick, have their
own self-contained state, and access to the world state. They can be enabled and
disabled dynamically.

**Commands** are (marker) structs that implement the `GameCommand` trait. These
are processed in the game loop after systems, in the order they were received by
the broker and inserted into the channel. Commands can really just be thought of
as a 'run once' system. Like systems, they have access to the world state, as
well as a token stream, representing the entire string of input data a player
sent.

All text player input is parsed as a command.

## Scripting

Rhai is a dynamic scripting language with a syntax similar to Rust, but not
nearly as verbose. Blossom uses Rhai as both the data format for entities like
rooms and items, but also as a way to implement commands and systems. Ideally,
most game content will be scripted in Rhai, parsed by the engine, and converted
into Rust structs, call Rust functions, and send back closures in cases like AI
behaviour.

This has a benefit of allowing users to keep the server running while they
develop proper features for it within Rhai, that can be added, enabled, or
disabled on the fly. In addition, with properly exposed Rust functions, it
should be very easy for users to create content without neccesarily having to
know how to code, or dive into a language as complex as Rust.

Currently it can only be used as a data format.
