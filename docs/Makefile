# Build the continuation guide.
#
# Requires tectonic (already installed):
#   cargo install tectonic          # or: https://tectonic-typesetting.github.io
#
# Tectonic downloads the TeX packages it needs on first run and caches them,
# so the first build takes a few minutes and later builds take seconds.

TEX     := tectonic
MAIN    := main.tex
PDF     := main.pdf
SOURCES := $(MAIN) $(wildcard tex/*.tex)

.PHONY: all watch clean open

all: $(PDF)

$(PDF): $(SOURCES)
	$(TEX) -X compile $(MAIN)

# Rebuild on every save. Needs `entr` (apt install entr).
watch:
	ls $(SOURCES) | entr -c $(TEX) -X compile $(MAIN)

open: $(PDF)
	xdg-open $(PDF)

clean:
	rm -f $(PDF) main.log main.aux main.out main.toc
