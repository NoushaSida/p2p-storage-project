#!/bin/bash
cd $HOME/cqlsh-6.8.41/bin/

check_output_contains() {
    local command="./cqlsh -e \"$@\""
    output=$(eval "$command" 2>&1)
    #echo $output
    if [[ $output == *"doesn't exist"* ]]; then
        return 0
    else
        return 1
    fi
}

echo "Cleaning database..."

command="DROP keyspace user;"
while ! check_output_contains $command; do
    sleep 2
done
echo "Dropped keyspace user."

command="DROP keyspace file;"
while ! check_output_contains $command; do
    sleep 2
done
echo "Dropped keyspace file."

command="DROP keyspace peer;"
while ! check_output_contains $command; do
    sleep 1
done
echo "Dropped keyspace peer."

echo "Database cleaned successfully."
