# Introduction

Todays way of writing, installing and running software has gone hilariously
slow and way too complex. I realised this when I was trying to build an app,
which should be capable of controlling any Windows, Linux or Mac device
with declarative configurations made by up the user. This is the entire idea
behind NixOS, but it wasn't enough for me. I needed something that could even
control the software itself, which is installed on any device. That's were
things become a little bit TOOOOOOO complicated...

Let's begin with programming languages:

The problem is quite simple to explain, but also simply impossible to solve: 
We have too many programming languages, for too many usecases, incapable of 
working together in a elegant way, without any drawbacks. 

Ever tried using ABI's from inside Java applications? - I did and realised,
it was a stupid idea. But like not the idea of being able to call
some libraries, rather the idea of thinking that Java was any suitable of 
offering a great integrations for ABI's.

You might think now: "well, just don't use Java. Just use Rust! It offers
maximum safety, performance and ergonomic environments for developers.".
You might be right that Rust maybe the right choice for this specific usecase, BUT
WE STILL HAVE PROJECTS THAT USE OTHER LANGUAGES AND THEY WILL NEVER BE
REWRITTEN, BECAUSE IT'S NOT WORTH THE EFFORT.

For now there are two ways of compiling and running software: **J**ust-**I**n-**T**ime and
**A**head-**O**f-**T**ime. With JIT-Compilation some code is just compiled before it's
execution should begin. AOT-Compilers compile the programm on a machine first and
create a binary file, which can be run on the same kernel with the same processor architecture and 
blablabla. I'm actualy tired of this, because both approaches have MAJOR DRAWBACKS, that many programmers
don't even think of anymore, **BUT** what if I tell you, that the true problems lie on
something else:

**THE KERNEL**, or more specifically, **TODAYS KERNELS**.

"You dare to question the Linux/Windows/Unix?/MacOS/BSDeeznuts/Redoxallwritteninrust kernel?" - Yes, I do :)

All those kernels base, some more, some less, on a file-system and so on files. Every file just
consists of data. Some data may be executable, so you can try to run it with the kernel your currently
on. Good luck, because the kernel doesn't care about what the file you're trying to run does.
The kernel feels and works like an extension to the CPU. Both CPU and Kernel
only speak machine code, so time to invent a language, which converts human-readable code
to machine code. It's just a mess and I don't even want to begin with regular files or drivers.

Introducing Crash - A kernel which speaks the Crash Programming language!
What's the difference to other kernels or languages? Well first of all you have 
to understand that the kernel works exactly like the language.
What do I mean with this?

Crash is purely functional, but more about that later.
Now think about files - Why don't we replace files with functions?
Why don't we make everything a function?! Let that sink in for a second..

[Let's jump to the programming language!](./language)
