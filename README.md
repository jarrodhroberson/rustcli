# Rust Command Line Experiment

This is just a little learning exercise on how to write a command line application in rust and manipulate the
filesystem.

It just walks the current directory and outputs some basic details about the files in that directory as JSON.

There are some more advanced things I had to research in here that might be helpful to someone else.

1. Automatically generating JSON from Structs with [serde](https://docs.rs/serde_json/latest/serde_json/).

2. Converting a u32 representation of unix permissions to the more familiar 3 digit octal representation.

3. Converting the unix timestamp u64 into an [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339) represenation so
   it is human readable.

4. Retrieving command line arguments with [Command Line Argument Parser for Rust](https://github.com/clap-rs/clap)