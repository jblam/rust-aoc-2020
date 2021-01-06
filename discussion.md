# Subjective discussion

Noting down some thoughts in case I ever want to write this up.

## This is a contrived exercise

Like Project Euler, the exercises are a bit contrived — not just in the sense of the "advent story", but in terms of the computational demands.

That's good! Nobody would enjoy this if you needed to spend hours performance-tuning or fretting over allocations. But, it does mean that the perf characteristics of your language are not super relevant.

One observation I'd make is "every problem needs an iterator," unlike most of my actual work where "every problem needs async-await." (Or "every problem is a database" perhaps.)

## On Rust:

For context, my perspective comes from 10-odd years of C#, 20-odd years of JavaScript, a few months pretty recently in embedded C++, a bunch of TypeScript, some long-ago Java, dabbling in various others (my first language was logowriter), and a casual passing interest in the general design and implementation of languages.

For all that, I guess I have to admit I'm a massive C# fanboy. C# is showing its age in some places, to be sure, but I find the tedious parts not-that-tedious, the pitfalls easy-to-avoid, the ecosystem rich and easy to navigate.

I've done part of Project Euler in Rust. I looked in to using Rust on embedded microcontrollers, but it's not ready for prod on the microcontrollers I need.

### I like Rust

In particular, I like the type system's distinction between ownership and borrowing, and the mutability of variables.

### But I still won't (can't?) use it

I've tried to use Rust before, but I can't stick with it because it's immature on the platforms I need (e.g. ESP8266 microcontroller), and it doesn't have a good GUI story for desktop.

I'd consider using it for server implementations, but I seldom need a server which shares no code with a GUI of some sort, which invariably leads me back to C#.

### No "generator" `yield` is disappointing

The nature of the AoC puzzles requires a *lot* of iterator operations. I find doing all the operations in closures a bit fraught: the closure syntax `|args| expr` is just that much harder to type when compared with C#, JS/TS, C++ (the `|` key being directly above `Shift` makes for an awkward 4th finger stretch), and the type system seems to struggle inferring the types if your code's not 100% right. The worst thing is if you leave off the closing bracket of the tuple destructure — `|(x, y|` — resulting in an amazing cascade of compiler errors, as it misinterprets the closing `|` and then rampages through the rest of your file screaming about the missing `)`.

I always found it odd that Rust got "async await" before it got "yield". I'm sure there's a good reason, of course; language features are not free.

### Lifetimes are really not a big deal for me

Lifetimes usually come first in the "what's hard about rust" polls. I don't find that an issue at all.

Usually my interactions with lifetimes are "you need to add `'_` to the `impl Iterator` return type. Occasionally I have to slap `'a` on references until things go away.

I assume things get more interesting if you need to tighten the belt on allocations. I have gone down some bad paths with, for example, trying to borrow the original `&str` input all the way through to the output , which leads me to—

### Rust's ampersand obsession irritates me

From the perspective of a GC-d, "object-y" language user (fluent in C# / JS, conversant in Java): my least favourite aspect of the _languages_ of C, C++, and Rust is the quantity of inane sigils `&`, `*`.

After working in C++ for a few months, my hope for Rust was that "reference twiddling" would not be such a big deal. I'm disappointed in how much time I have to spend keeping track of the "level of reference'd-ness". In particular, as a _consumer_ of iteration methods, it feels "pretty random" whether you'll get `T`, `&T`, `&&T`.

Further, "automatic dereferencing" feels inconsistent to me, in that _sometimes_ Rust will "do what I mean" when I have a variable of the "wrong" reference'd-ness, but _often_ it will not. It's tedious to manually dereference operands in `==` or `+`. It's especially tedious to dereference numeric literals.

My overall impression is that the `&&` stuff is "noise" and doesn't help with correctness. I have consistently "made it compile by adding `&`" then come back and identified correctness issues related to reference-vs-value later.

I totally admit that I'm spoiled by C# in this regard, which again leads me to—

### Derive is tedious

In a similar way, `#[derive(Debug, Clone, Copy, PartialEq)]` does not delight me. I spend a lot of time zipping around the codebase adding this to struct definitions in order to "make the squiggly underline go away".

Possibly, rust-analyzer could provide a code action to automatically derive the relevant trait when needed?

This and the previous are especially linked in my mind, because C# draws a distinction between reference-types (a variable of which always points to storage of a reference) and value-types (a variable of which always points to storage of the actual data). So, morally-speaking, at least one of `Clone` and `Copy` are always implemented.

### Turbofish is easy to forget and hard to type

Good old `.collect::<Vec<_>>()`. Or as it usually comes out, `.collect()::Vec<(>`, where rust-analyzer's autocompletion, VSCode's bracket insertion, and the ergonomics of the US-QWERTY keyboard all fight to make this idiom as difficult as possible.

My understanding is that Rust "needs" `::<>` to make parsing easier? I'd be more sympathetic if it were able to emit a succinct "you forgot the turbofish" error.

Also, as a C# fanboy, I find return type variance a bit unsettling; I'm never really sure if I even _want_ to write `let x = some_str.parse()` rather than `let x = int32::from_str(some_str)`.

### Dead code warnings

I think I can understand the rationale, but my typical experience feels like

> **Rustc:** My dude, that code you just spent 20 minutes writing is unused. [Compile warning]
>
> **Rustc:** Hey just letting you know you wrote another unused method. [Compile warning]
>
> **Rustc:** FYI you just imported a submodule full of unused things. Here's a list of all those things. [12 Compile warnings]
>
> **me:** *finally implements enough that I can call something from `main`*
>
> **Rustc:** Oh hey you *are* using that! Oopsie!

I don't like looking at bad compile warnings.

## On VS Code

Being a C# fanboy goes very well with being a Visual Studio (purple) fanboy. Visual Studio Code (blue) always felt a bit "cheap" in ways that are hard to describe. Partly, looking at Chromium trying to render text on Windows, I suppose?

### Debugging

With C# on Visual Studio, everything is "right there" and you can just set breakpoints and press F5.

To set up debugging in VSCode, you need to create a Run Task, which involves manually editing a JSON file and then interacting with a C++ extension, which definitely doesn't feel polished. For a long time I was convinced I couldn't debug tests, only the actual program starting with `main`.

### Automatic parenthesis completion

Feels almost always wrong, though I expect if I turn it off I'll find out it's much closer to 50-50.

I very often select text and want to overwrite it; VSCode thinks typing a bracket over a selection means "surround the text" which I cannot get my head around. I can't think what problem that actually solves.

### Git integration

I absolutely cannot get in to "nonvisual source control management". I *need* to see the graph.

It seems unusually hard to run a workflow like:

- create sidebranch
- commit some commits
- merge (no ff) sidebranch into main branch
- create new sidebranch

The defaults are configured in ways which seem very obnoxious to that.

### Rust analyzer checks on save

In Visual Studio, the Intellisense works "live" to such a degree that I can go for a _long_ time without saving. Rust-analyzer usually only updates on save, which is an occasional papercut.

Also, a few classes of lifetime error don't display nicely through rust-analyzer / VSCode. It's not obvious that you get different (and very pretty!) output from rustc / cargo.