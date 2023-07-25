```bash
# runs all tests beside the as ignored flaged tests
cargo test

# runs only the ignored tests
cargo test -- --ignored --show-output

# runs only the ignored tests with docker in the name
cargo test docker -- --ignored --show-output
```