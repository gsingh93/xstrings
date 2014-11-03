all: xstrings

xstrings: xstrings.rs
	rustc xstrings.rs

clean:
	rm -f xstrings
