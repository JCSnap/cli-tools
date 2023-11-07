# CLI Tools
This is a collection of my cli tools for my terminal, built with rust for me to explore rust. This guide will assume you have knowledge about git cloning, and that you are a Mac user.

## Tools
### checkduplicates
Check for file duplicates in the directory stated. For example, use the following command
```
checkduplicates ~/Downloads
```
to list out all the duplicate files in your Downloads folder. This command will let you know which files have duplicates and how many duplicates there are.


## Setting up
After git cloning this project. Build it with
```
cargo build --release
```
A target file would be created in your current directory with the executables. Then, use the command
```
sudo mv target/releases/checkduplicates /usr/local/bin/
```
and then
```
sudo chmod +x /usr/local/bin/checkduplicates
```
to make checkduplicates executable regardless of where you are.  
This means that suppose you are in the ~/Downloads location, you could run the following:
```
checkduplicates .
```

## TODO
- Command to remove duplicates
- Git repositories analyzer
- Markdown to HTML converter
