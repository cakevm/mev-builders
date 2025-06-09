# MEV Builders
Sick of maintaining builder RPCs yourself in your bot? This repository has you covered! A list of MEV builders with their RPCs and other useful information. No extra dependencies, no build script, just a simple list of builders ordered by landed blocks. 

# Status
This is the first draft of the repository. It is not yet complete and may contain inaccuracies. Feel free to contribute by adding or updating builders.

# Usage
Just import the list, filter what you need, order them by your preference, and use them in your bot.

```rust
use mev_builders::BUILDERS;
```

# Contributing
If you want to add, or update a builder, please open a pull request.

# Acknowledgements
The `mev_share_rpc` is based on https://github.com/flashbots/dowg

## License
This project is licensed under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).