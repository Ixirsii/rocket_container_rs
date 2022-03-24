Rocket Stream Rust
==================

Rocket Stream Rust is a solution for [Bottle Rocket Studio's][1]
[Rocket Stream][2] coding challenge written in Rust. It is also my first 
attempt to learn the Rust programming language.

## Dependencies ##

- [backoff][3] - For exponential backoff and retry
- [reqwest][4] - For making HTTP requests
- ~~[Rocket][5] - For HTTP request routing~~
- [serde][6] - For (de)serialization
- [serde_json][7] For (de)serialization to/from JSON

[1]: https://www.bottlerocketstudios.com/careers
[2]: https://bottlerocketstudios.stoplight.io/docs/rocket-container/ZG9jOjYzMzI0-welcome
[3]: https://docs.rs/backoff/0.4.0/backoff/index.html
[4]: https://docs.rs/reqwest/0.11.10/reqwest/index.html
[5]: https://rocket.rs/
[6]: https://docs.serde.rs/serde/index.html
[7]: https://docs.serde.rs/serde_json/index.html
