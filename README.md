# Rust Axum Demo

🚀 Welcome, thrill-seekers and code connoisseurs, to an electrifying showcase of cutting-edge technology! Behold, the dynamic duo of Rust 🦀 and Axum! 🦾💥 Prepare to be dazzled by lightning-fast performance, fortified by ironclad safety, and immersed in a web framework that's as sleek as a supersonic jet. So fasten your seatbelts and hold on tight, because this demo will take you on a wild, adrenaline-pumping ride through the world of Rust and Axum! 🌐🎉 Let's unleash the blazingly fast power of fearless programming and turbocharged web development together! 💻⚡️

## Run the demo
 
- [Install Rust](https://rustup.rs/)
- Install the `protoc` protobuf compiler  
  ```sh
  # Ubuntu
  sudo apt install protobuf-compiler libprotobuf-dev
  # Fedora / CentOS / ...
  sudo dnf install protobuf-compiler
  # MacOS
  brew install protobuf
  # Windows
  open "https://ubuntu.com/tutorials/install-ubuntu-desktop#1-overview"
  ```
- Create `.env`  
  `cp sample.env .env`
- Start the database and tracing collector  
  `docker compose up -d`
- Apply the migrations  
  `cargo run -- migrate`
- Run the server  
  `cargo run -- serve`
- Play around with the endpoints
  - Create a link  
    `curl -i -X POST 'http://localhost:42069/api/links' -H "Content-Type: application/json" -d '{"url":"https://github.com/FreakyBytes/rust-axum-demo","code":"foo"}'`
  - Create a link with random code  
    `curl -i -X POST 'http://localhost:42069/api/links' -H "Content-Type: application/json" -d '{"url"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}'`
  - "Visit" a link  
    `curl -i 'http://localhost:42069/api/links/foo'`
  - See meta info of a link  
    `curl -i 'http://localhost:42069/api/links/foo/meta'`
  - Fetch metrics  
    `curl -i 'http://localhost:42069/metrics'`
- Open Jäger and look at some of those nice traces: localhost:16686/search?limit=20&lookback=1h&maxDuration&minDuration&service=rust-axum-demo&tags={"http.scheme"%3A"HTTP"}

## Develop the demo

- Do the [above](#run-the-demo)
- Install `rust-analyzer`
- Configure your editor to use `clippy` as check command  
  (In VSCode set `{ "rust-analyzer.check.command": "clippy" }`)
- Install the sqlx cli  
  `cargo install sqlx-cli`
- Ensure the database is up while you develop  
  (at least while you develop DB queries, so sqlx can buid-time check your queries and types)
- Run `cargo sqlx prepare` to cache validated statements  
  (good thing todo before committing, so the CI does not need a database running)
- When developing migrations (and breaking `cargo run -- migrate`) you can also apply the migrations with `cargo sqlx migrate run`
- Use [bacon](https://github.com/Canop/bacon) or [cargo watch](https://watchexec.github.io/#cargo-watch) to recompile/check/lint/whatever on code changes
