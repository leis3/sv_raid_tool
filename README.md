[![Deploy](https://github.com/leis9631120/sv_raid_tool/actions/workflows/pages.yml/badge.svg?branch=main)](https://github.com/leis9631120/sv_raid_tool/actions/workflows/pages.yml)

# sv_raid_tool


## Usage

```
$ git clone https://github.com/leis9631120/sv_raid_tool.git
$ cd sv_raid_tool
```

## Build

```
$ wasm-pack build --out-dir ./static --out-name wasm --target web
```

## Run

Start the server with `miniserve`, etc.

```
$ cargo install miniserve   
$ miniserve ./static --index index.html
```