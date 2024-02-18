# Rust in Action Book


## Goals

- Dangling pointers
- Data races
- Buffer overflows
- Iterator invalidation


When programs are compiled in debug mode, Rust also protects against integer overflow. 
What is integer overflow? Well, integers can only represent a finite set of numbers; 
these have a fixed-width in memory. Integer overflow is what happens when the integers 
hit their limit and flow over to the beginning again.
