# Arc_API_rs

This is a client web API that connects to the Metaforge endpoints for Arc Raiders. The data returned from this tool is sourced from [MetaForge](https://metaforge.app/arc-raiders). 

> Notes: As per MetaForge documentation, this crate has built-in rate-limiting. Caching is expected to be handled in projects utilizing this crate.

## Examples

Available Example Targets:
- events_schedule
- game_map_data
- list_arcs
- list_items

Ex.

`cargo run --example list_arcs`

## Tests

Available Test Targets:
- arcs
- events
- items
- maps
- pagination
- rate_limit

Ex.

`cargo test --test arcs`

## License

This project is licensed under the [MIT License](LICENSE).

### Third-party licenses

This crate depends on third-party libraries. See [`LICENSES.html`](LICENSES.html) for a full list of dependency licenses, generated via [`cargo-about`](https://github.com/EmbarkStudios/cargo-about).
