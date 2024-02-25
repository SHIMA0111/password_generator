# Password Generator
This is password generator developed by Rust with rand crate. 

# Installation
1. Build the application: ```cargo build --release```
2. Execute the application file on `target/release`
    - On Mac, the file named as `password_generator`
    - On Windows, the file named as `password_generator.exe`

# Options
| Option (Short) | Effect                                                     | Default | 
|----------------|------------------------------------------------------------|---------|
| --length (-l)  | Password Length (this parameter limited upon 5 or greater) | 12      |
| --num-of-passwords (-n)| Number of the passwords                                    | 1       |
| --exclude-symbols (-e) | Specify exclude symbols from the password generation(※)    | None    |

※ The default symbols are `~!@#$%^&*()_+-={}[]|:;"<>,.?\/` so you can input 
what symbols will be removed from generated passwords via `-e` option.

# Usage Example
```shell
./target/release/password_generator -l 15 -n 2 -e '@#>\'
```
Returns
```
lqw75kybwny^zRj
Q)6pvX68KXK-=}9
```

# License
[MIT License](/LISENSE.md)