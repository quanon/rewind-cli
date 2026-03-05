# rewind-cli

CLI tool to fetch yo-yo product information from [YoYo Store Rewind](https://yoyostorerewind.com/) as JSON.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# Coming soon (近日入荷)
rewind list coming-soon

# New items / Restocked (新商品/再入荷)
rewind list new-items
```

### Output

```json
[
  {
    "vendor": "UNPRLD",
    "title": "クラリティ",
    "price": 13900
  },
  {
    "vendor": "C3yoyodesign",
    "title": "スピーダホリックXX",
    "price": 4990
  }
]
```

| Field    | Description        |
|----------|--------------------|
| `vendor` | Brand name         |
| `title`  | Product name       |
| `price`  | Price in JPY (int) |

## Requirements

- Rust 1.75+
