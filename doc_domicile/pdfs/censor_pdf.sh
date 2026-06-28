#!/usr/bin/env bash

# Arguments
# $1 - Input PDF
# $2 - Watermarking PDF
# $3 - Output PDF

set -x

TEMP_PDF='temp.pdf'

# Watermark
pdftk "$1" multistamp "$2" output "$TEMP_PDF"

# Rasterise
gs -dNOPAUSE -dBATCH -sDEVICE=pdfimage24 -r200 -o "$3" "$TEMP_PDF"

# Cleanup
rm "$TEMP_PDF"
