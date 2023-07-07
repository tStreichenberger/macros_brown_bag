# Expand all macros in the src code and put it into a new file

cargo +nightly rustc --profile=check -- -Zunpretty=expanded > expanded.rs