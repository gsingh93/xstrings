Extended Strings [![Build Status](https://travis-ci.org/gsingh93/xstrings.svg?branch=master)](https://travis-ci.org/gsingh93/xstrings)
================

This program searches the given file for binary strings, hex strings, and base64 strings. It was created for use in [CTF](https://ctftime.org/ctf-wtf/) competitions, but may have other uses.

To build, run `cargo build`. `./target/xstrings -h` to see a list of command line options.

TODO
----
1. Allow user to customize the match length (i.e. only binary strings longer than five characters).
2. Implement searching for words. Allow the user to optionally specify a path to a dictionary as an argument to -w. Package default dictionary with program.
3. Add an option for not showing duplicate matches. A string could be a valid binary string as well as a valid base64 string. We should add an option to only show unique strings (or make this default and add a --verbose flag to show all duplicates).
4. Add support for searching through multiple files.
5. Implement filters (i.e. output all strings containing only letters).
