# Rasterising a PDF

## Watermark
```
pdftk input.pdf multistamp watermark_template.pdf output output.pdf
```

## Rasterise
```
gs -dNOPAUSE -dBATCH -sDEVICE=pdfimage24 -r300 -o NAME-rasterised.pdf NAME.pdf
```
