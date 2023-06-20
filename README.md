# Hypixel Rust Backend &emsp; [![Build Status]][actions] [![Discord]][discord link]

[actions]: https://github.com/kr45732/rs-pixel-backend/actions?query=branch%3Amain
[Build Status]: https://img.shields.io/github/actions/workflow/status/kr45732/rs-pixel-backend/ci.yml?branch=main
[Discord]: https://img.shields.io/discord/796790757947867156?color=4166f5&label=discord&style=flat-square
[discord link]: https://dsc.gg/skyblock-plus

## Set Up
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Hypixel API Key](https://api.hypixel.net)

### Steps
- Clone the repository
- Rename the `.example_env` file to `.env` and fill out required fields **OR** set required fields using environment variables
- Run `cargo run --release` (may take some time to build)
- Use the API!

### Required Configuration Fields
- `BASE_URL`: Base address to bind to (e.g. 0.0.0.0)
- `PORT`: Port to bind to (e.g. 8000)
  - Online hosts will automatically set this
- `API_KEY`: Your Hypixel API key

## Usage
### Endpoints
- /key
- /boosters
- /leaderboards
- /punishmentstats
- /player
  - username
  - uuid
- /guild
  - id
  - name
  - player
  - username
- /counts
- /status
  - username
  - uuid
- /recentGames
  - username
  - uuid
- /skyblock/profiles
  - username
  - uuid
- /skyblock/profile
  - profile
- /skyblock/bingo
  - username
  - uuid
- /skyblock/news
- /skyblock/auction
  - player
  - uuid
  - profile
  - username
- /skyblock/auctions
  - page
- /skyblock/auctions_ended
- /skyblock/bazaar
- /skyblock/firesales
- /resources/{resource}/{sub_resource}

### Documentation & Examples
- Coming soon

## Free Hosting
### Deploy On Railway
[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/new/template?template=https://github.com/kr45732/rs-pixel-backend&envs=BASE_URL,API_KEY&BASE_URLDesc=The+base+URL+of+the+domain.+Do+not+modify+this&API_KEYDesc=Your+Hypixel+API+key&BASE_URLDefault=0.0.0.0&referralCode=WrEybV)

### Deploy On Gigalixir
Steps to deploy on [Gigalixir](https://gigalixir.com/):
1. Clone repository
2. Install gigalixir CLI: `pip3 install gigalixir`
3. Sign up: `gigalixir signup`
4. Create app: `gigalixir create -n NAME`
5. Set environment variables: `gigalixir config:set key=value`
6. Deploy app: `git push gigalixir`
7. Acess at [https://NAME.gigalixirapp.com/](https://NAME.gigalixirapp.com/)

# License & Contributing
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

# Note: this is still a work in progress