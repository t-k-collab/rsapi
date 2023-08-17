# rsapi

## Cargo test

 `cargo test -v -- --nocapture`
 
## Cargo watch

### Install
``` sh
cargo install cargo-watch
```

### Usage
`cargo watch -x run`

`cargo watch -x test`


### Docker

``` sh
docker-compose up api
```

### PostgreSQL

#### Local

`brew services start/stop/restart postgresql@14`

### install sqlx-cli
`cargo install sqlx-cli --no-default-features --features rustls,postgres`

to create `.sqlx`,
`cargo sqlx prepare`