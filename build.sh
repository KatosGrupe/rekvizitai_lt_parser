#!/usr/bin/env sh

LIBCLANG_PATH="/gnu/store/yn66j95ra1x62ki8ijm18w7r4n67hpq5-profile/lib" \
PKG_CONFIG_PATH="/gnu/store/6ac664rxxcr5ysypbymfridadss6lwg0-leptonica-1.83.1/lib/pkgconfig:$PKG_CONFIG_PATH" \
PKG_CONFIG_PATH="/gnu/store/xvclcj6rgc7kkdnjjxr5a5mqm09ramg1-tesseract-ocr-5.3.0/lib/pkgconfig:$PKG_CONFIG_PATH" \
cargo build $@
