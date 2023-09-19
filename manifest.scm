;;; GNU Guix manifest to set the development environment
;;;  guix shell -m manifest.scm

(specifications->manifest
  '("clang"
    "tesseract-ocr"
    "tesseract-ocr-tessdata-fast"
    "leptonica"))
