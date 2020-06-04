## My attempt at the Rust Book's minigrep project.

``` Bash
# Case sensitive search - Rust stdlib string contains function
cargo run pattern text_file.txt
# KMP case sensitive search - Use custom string search
USE_KMP=1 cargo run pattern text_file.txt
# KMP case insensitive search
USE_KMP=1 CASE_INSENSITIVE=1 cargo run pattern text_file.txt
```
