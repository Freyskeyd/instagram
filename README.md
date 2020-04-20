# Rust Instagram API

This library aims to offer an access to the `web api` and the `private api` for
Instagram. This is not a bot or anything like that, but you can build
interactive software with it.

  - **Status**:  Under developpement.

*All the next section needs to be improved and can changes quickly*

## Installation

Just add `instagram = "0"` to your `Cargo.toml`

## Configuration

For now their is no particular configuration.

**Tests** requires that you define both `INSTAGRAM_USERNAME` and
`INSTAGRAM_PASSWORD` if you want to implement new functionality that need to hit
Instagram. Best way is to create a fixture of the calls.

## Usage


You have two options, the `web` API and the `private` API.

```rust
extern crate instagram;

use instagram::web_api::Client;

fn main() {
  Client::new().user_infos("Some_acocunt").await.unwrap();
}
```

## Known issues

## Getting help

If you have questions, concerns, bug reports, etc, please file an issue in this repository's Issue Tracker.
