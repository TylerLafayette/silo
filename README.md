# <img src="docs/silo-logo.png" width="100" />
Silo is a decoupled, fast, and smart research and analysis engine for storing and querying data about tagged populations using tags stored on a tree structure.  
  
:warning: **WARNING**: Silo is currently not functional, and is very much a WIP. I can make no guarantees of code quality or consistency at the moment.
## Structure
```js
•
|- silo           // Silo's CLI entrypoint, spawns the core's Service with all default actors
|- silo-core      // Silo's domain, util functions and default actix system
|- silo-db        // Silo's database adapters, provides methods for persisting data in Silo
|- silo-transform // Silo's exporting interface, provides different output types and methods for extracting data to other places
```
## Building and running
Build Silo's CLI (`silo`) with:
```bash
$ cargo build --release --bin silo
```
Run through cargo with:
```bash
$ cargo r --release --bin silo
```
## Testing and documentation
To run silo's test suite, use:
```bash
$ cargo test
```
Build the docs (with the optional `--open` flag to open them):
```bash
$ cargo doc --open
```
