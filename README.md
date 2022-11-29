# Description

RestAPI service to scrape https://rekvizitai.vz.lt page info for clients and return as JSON.

# Requirements

- Leptonica
- Tesseract
- Clang


# Compilation

- Install requirements: `sudo apt-get install libleptonica-dev libtesseract-dev clang`
- Use cargo to build the executable `cargo build`

# Use

- POST request at endpoint at /extrator with body
``` json
 {'url': 'https://rekvizitai.vz.lt/imone/...'}
```
- Use --help flag for additional info

# Testing

None
