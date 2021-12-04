# Software Engineer III Take Home Assessment

## Requirements

### Rust

The project has been built & tested with version 1.57.0 (also set in the
`rust-toolchain` file for use with `rustup`).

### Node.js

The project has been built & tested with version 16.13.1 (also set in the
`.nvmrc` for use with `nvm` & compatible Node version managers).

#### Yarn

Node dependencies are managed with Yarn. After [installing Yarn][install-yarn],
the dependencies can be installed by running the following from the root of the
project:

```bash
cd ui && yarn install
```

[install-yarn]: https://classic.yarnpkg.com/lang/en/docs/install/ "Yarn installation instructions"

## Development

### UI (Vue.js)

The development Vue.js server can be started by running the following from the
root of the project:

```bash
cd ui && yarn serve
```

This will start the development server with hot-reloading, and output the
URL(s) it can be reached on (typically `http://localhost:8080`).

### Rust

The HTTP API defaults to serving on `http://127.0.0.1:8000`, and can be changed
by setting environment variables described in the
[Configuration](#configuration) section.

## "Production"

The UI can be built at the same time as the API by setting the `BUILD_UI`
environment variable to have any value when running `cargo build`.

```bash
env BUILD_UI=true cargo build --production
```

This will automatically run `yarn build` in the `ui` directory, with the Vue.js
app automatically set up to be served from `http://0.0.0.0:8000/ui/` by the
Rust application.

Requesting `/` from the Rust app, will redirect to `/ui/index.html`.

## Configuration

| Environment Variable | Default                                       |
|----------------------|-----------------------------------------------|
| `BIND_ADDRESS`       | `127.0.0.1`                                   |
| `BIND_PORT`          | `8000`                                        |
| `RUST_LOG`           | `systeminit_take_home=debug,tower_http=debug` |

