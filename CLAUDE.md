# CLAUDE.md

## Project Overview

`rewind` is a CLI tool that fetches yo-yo product information from [YoYo Store Rewind](https://yoyostorerewind.com/) and outputs it as JSON.

## Tech Stack

- Language: Rust (edition 2024)
- Dependencies: clap (CLI parsing), reqwest (HTTP), serde/serde_json (JSON)
- Data source: Shopify JSON API (`/collections/{slug}/products.json`)

## Build & Run

```sh
cargo build
cargo run -- coming-soon   # 近日入荷
cargo run -- new-items     # 新商品/再入荷
```

## Architecture

- Single binary (`src/main.rs`)
- Uses Shopify's built-in JSON API instead of HTML scraping for reliability
- Outputs JSON array of `{ vendor, title, price }` objects

## Supported Collections

| CLI argument   | Collection URL slug | Description       |
|----------------|---------------------|-------------------|
| `coming-soon`  | `coming-soon`       | Coming soon items |
| `new-items`    | `new-items`         | New / restocked   |
