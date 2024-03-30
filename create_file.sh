#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 <file_name>"
  exit 1
fi

file_name="src/$1.rs"
main_file="src/main.rs"

# Check if file already exists
if [ -e "$file_name" ]; then
  echo "File $file_name already exists. Please choose a different name."
  exit 1
fi

# Create the Rust file
echo "pub fn run() {" > "$file_name"
echo "    println!(\"$1.rs\");" >> "$file_name"
echo "}" >> "$file_name"
echo "Created $file_name"

# Add mod statement to main.rs
if ! grep -q "mod $1;" "$main_file"; then
  # Insert mod statement at the beginning of the file
  sed -i "1i mod $1;" "$main_file"
fi

main_file="src/main.rs"

sed -i '/^    \/\/ /!s/^    /    \/\/ /' "$main_file"

echo "Commented out all uncommented lines in main function of $main_file"