# Warpgrapher + Rocket
![Build Status](https://github.com/warpforge/warpgrapher-rocket/workflows/Test/badge.svg)](https://github.com/warpforge/warpgrapher-rocket/actions?query=workflow%3A%22Test%22+branch%3Amaster)

This project demonstrates how to run a [warpgrapher](https://github.com/warpforge/warpgrapher) service on a [rocket](https://github.com/SergioBenitez/Rocket) server. 

### External Requirements

- Running neo4j database:

```bash
export WG_NEO4J_HOST=127.0.0.1
export WG_NEO4J_PORT=7687
export WG_NEO4J_USER=neo4j
export WG_NEO4J_PASS=*MY-DB-PASSWORD*
```

```bash
docker run --rm -p 7687:7687 -e NEO4J_AUTH="${WG_NEO4J_USER}/${WG_NEO4J_PASS}" neo4j:4.1
```

### Dependencies

Rust nightly:

```bash
rustup install nightly
rustup override set nightly
```

### Run

Run app in rocket server:

```bash
cargo run
```
