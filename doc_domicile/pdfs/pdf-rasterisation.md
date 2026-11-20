# Rasterising a PDF

## Censor
- Use Okular
- Black highlighter, 100% opacity
- **MAKE SURE TO RASTERISE PDF WHEN SAVING, DOESN'T DESTROY TEXT OTHERWISE**

## Remove Pages
```
pdftk input.pdf cat 1-20 output output.pdf
```

## Watermark
```
pdftk input.pdf multistamp watermark_template.pdf output output.pdf
```

## Rasterise
```
gs -dNOPAUSE -dBATCH -sDEVICE=pdfimage24 -r300 -o NAME-rasterised.pdf NAME.pdf
```
