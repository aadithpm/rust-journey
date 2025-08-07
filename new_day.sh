#!/bin/bash

# Find the highest numbered day folder
highest_day=0
for dir in days/day_*; do
    if [ -d "$dir" ]; then
        # Extract the number from the directory name
        day_num=$(echo "$dir" | sed 's/days\/day_//')
        if [ "$day_num" -gt "$highest_day" ]; then
            highest_day="$day_num"
        fi
    fi
done

# Increment to get the next day number
next_day=$((highest_day + 1))
new_dir="days/day_$next_day"

# Create the new directory
mkdir -p "$new_dir"

# Initialize Cargo project in the new directory
cd "$new_dir"
cargo init --name "day_$next_day" .
cd - > /dev/null

# Create ref.md with parameters as a markdown list
if [ $# -gt 0 ]; then
    echo "# References" > "$new_dir/ref.md"
    echo "" >> "$new_dir/ref.md"
    for param in "$@"; do
        echo "- $param" >> "$new_dir/ref.md"
    done
else
    echo "# References" > "$new_dir/ref.md"
    echo "" >> "$new_dir/ref.md"
    echo "- No references provided" >> "$new_dir/ref.md"
fi

echo "Created $new_dir with Cargo project and ref.md"