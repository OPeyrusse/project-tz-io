# project-tz-io
Project TIS-Zen I/O
===================

This project is lead by many eductational goals.

**TL;DR;**

 - Build a language similar to the one in TIS-100, with the flexibility of Shenzhen I/O to combine as many
   nodes as wanted.
 - Write a compiler for this language in Rust. Planning to use [nom](https://github.com/Geal/nom).
 - Write the same compiler in Clojure. Planning to use [instaparse](https://github.com/Engelberg/instaparse).
 - Produces class files from the parsed programs.
 - Produces LLVM pseudo-assembly code, for comparison with the JVM capabilities.

**Full story**

The original goal is to investigate the format of Java class files, writing one's own files. This requires 
knowledge of the internals of the JVM, its capabilities and so on.
For that purpose, I needed a language to parse, hopefully simple enough. Designing a language inspired from
TIS-100 should be enough, particularly as it implies so sort of distributed programing. I wanted to add the
flexibility of Shenzhen I/O to combine nodes as wanted.
After investigating LLVM project, I decided that compiling the same language to LLVM IR should be educative.
Regarding the language for the compiler, I wanted both to learn Rust and Clojure. So why not comparing the 
complexity of writing the same complex program in both languages.
