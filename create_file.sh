#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 <file_name>"
  exit 1
fi

file_name="src/$1.rs"
main_file="src/main.rs"

# Create the Rust file
echo "pub fn run() {" > "$file_name"
echo "    println!(\"{}{}{}$1.rs{}{}{}\", \"🦀\", \"🦀\", \"🦀\", \"🦀\", \"🦀\", \"🦀\");" >> "$file_name"
echo "}" >> "$file_name"
echo "Created $file_name"

# Add mod statement to main.rs
if ! grep -q "mod $1;" "$main_file"; then
  # Insert mod statement before fn main()
  sed -i "/fn main() {/i mod $1;" "$main_file"
fi
