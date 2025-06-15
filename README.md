# MEV Builders
Sick of maintaining builder RPCs yourself in your bot? This repository has you covered! A list of MEV builders with their RPCs and other useful information. Just a simple list of builders ordered by landed blocks. 

# Status
This is the first draft of the repository. It is not yet complete and may contain inaccuracies. Feel free to contribute by adding or updating builders.

# Usage
Just import the list, filter what you need, order them by your preference, and use them in your bot.

```rust
use mev_builders::BUILDERS;
```

By default, the list is ordered by the number of landed blocks in the last week. If a builder has no landed blocks, the builder will be moved to `OTHER_BUILDERS` list.

# Contributing
If you want to add, or update a builder, please open a pull request.

# Acknowledgements
The `mev_share_rpc` is based on https://github.com/flashbots/dowg. Many thanks to the Flashbots team for https://www.relayscan.io. The data is used to update the order of builders every week.

## License
This project is licensed under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).