# CLI Tools
This is a collection of my cli tools for my terminal, built with rust for me to explore rust. This guide will assume you have knowledge about git cloning, and that you are a Mac user.

## Table of Content
- [Tools](#tools)
    - [checkduplicates](#checkduplicates)
    - [askgpt](#askgpt)
- [Setting up](#setting-up)
- [Future plans](#future-plans)
- [Potential issues](#potential-issues)

## Tools
Here are the descriptions of the various tools in this repository and how to use each of them.

### checkduplicates
Check for file duplicates in the directory stated. For example, use the following command
```
checkduplicates ~/Downloads
```
to list out all the duplicate files in your Downloads folder. This command will let you know which files have duplicates and how many duplicates there are.

### askgpt
Ask questions to chatgpt in the terminal, optimised for short answers for questions about command line usage.
```
askgpt [QUESTION]
```
For example:
```
askgpt how to find and replace in vim
```

**Note**: make sure you do the following step before you `cargo build --release` to set up your openai api key, which is required for this command.  
You can use vim or any text editor.
```
cd ~
vim .env
```
Insert your api key:
```
OPENAI_API_KEY="YOUR OWN API KEY"
```

If you do not wish to set up your .env in your home directory, you can set it wherever you want, just make sure that change the path in `cli-tools/src/bin/askgpt.rs` to retrieve the key from your exact path. Once you have set up your api key, you can now proceed to the [Setting up](#setting-up) section.

## Setting up
Git clone this project. Then build the project (Note that some commands like askgpt has special instructions to set up before you can proceed with the following step):

```
cargo build --release
```
A target file would be created in your current directory with the executables of all the different commands. Then, move the executables to the `/usr/local/bin` location so that it can be executated all across your terminal instead of just in this repository. Suppose you want to move the checkduplicates executable, do 
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
to check duplicates in your current directory.

## Future plans
This project is not done yet. It is meant to accumulate more and more commands for my terminal. Some potential expansions include:
- Command to remove duplicates
- Git repositories analyzer
- Markdown to HTML converter

## Potential issues
Sometimes, your commands might not work as intended after you moved them to `/usr/local/bin/`. One potential reason is that you have a conflicting command with the same name. For example, suppose you moved askgpt to `/usr/local/bin`, you can check if there is a conflict with:
```
which -a askgpt
```
The expected output should only be one path showing `/usr/local/bin/askgpt`. However, if there are more than one path printed, that means you have conflicts. To resolve this, you can change the name of the command to something else that hopefully does not conflict with anything with:
```
sudo mv /usr/local/bin/askgpt /usr/local/bin/[NEW_NAME]
```
Of course, that means your command would change as well. For me, I have gone through the trouble of wondering why `openai` and `chat` do not work, before settling with the name `askgpt`.
