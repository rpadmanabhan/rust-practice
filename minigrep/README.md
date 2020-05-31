## My clone of the Rust Book's minigrep project. 

My addition: I added the option to do KMP string search by indexing the pattern only once.

``` Bash
# Case sensitive search
cargo run pattern text_file.txt
# KMP case sensitive search
USE_KMP=1 cargo run pattern text_file.txt
# KMP case insensitive search
USE_KMP=1 CASE_INSENSITIVE=1 cargo run pattern text_file.txt
```
