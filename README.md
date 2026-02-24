# Arc_API_rs

This is a client web API that connects to the Metaforge endpoints for Arc Raiders. The data returned from this tool is sourced from [MetaForge](https://metaforge.app/arc-raiders). Also, this is my first project, any feedback is much appreciated.

> Notes: As per MetaForge documentation, this crate has built-in rate-limiting. Caching is expected to be handled in projects utilizing this crate.

## Examples

Available Example Targets:
- events_schedule
- game_map_data
- list_bots
- list_items
- list_quests
- list_traders

Ex.

`cargo run --example list_bots`

## Tests

Available Test Targets:
- bots
- events
- items
- maps
- pagination
- quests
- rate_limit
- traders

Ex.

`cargo test --test bots`

## License

This project is licensed under the [MIT License](LICENSE).

### Third-party licenses

This crate depends on third-party libraries. See [`LICENSES.html`](LICENSES.html) for a full list of dependency licenses, generated via [`cargo-about`](https://github.com/EmbarkStudios/cargo-about).
