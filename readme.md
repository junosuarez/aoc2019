# advent of code 2019

i'm starting it a little late.

i intend to try to complete it in rust.
(i have never programmed a line of rust in my life).

i will complete each problem in a directory corresponding to the day,
and i will attempt to take notes here as i go.

## day 1

time to get going. i:

- i made this repo
- i installed rust following https://www.rust-lang.org/learn/get-started
- i installed the rust (rls) vscode extension (`ext install rust-lang.rust`). I want to learn [more of their tricks](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) as i go

i tried running `cargo new 1` but learned names starting with numbers are not supported by default. i had planned on having my directory names be simply the numbers, but i'd rather not use non-standard names so i reran it as `cargo new day`. this made a directory named `/day1` containing a `Cargo.toml` file (package manifest, similar to a package.json) and a `src` directory with a `main.rs` file containing a hello world function. that's nice!

the rust getting started page said i should be able to run `cargo run` to compile and run (this is nice), but doing so gave me a compiler error:

<details>
λ cargo run
   Compiling day1 v0.1.0 (/Users/jsuarez/Code/junosuarez/aoc2019/day1)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-m64" "-L" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f.28uhpcvvyylmwhbl.rcgu.o" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f.36twn0hwi95yrig6.rcgu.o" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f.3lita5i32ccy2qk7.rcgu.o" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f.4lnrnl0p3yiky0dr.rcgu.o" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f.4tyd8ifqe0jn4wrq.rcgu.o" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f.zyo6vssj0wl6o7z.rcgu.o" "-o" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps/day1-eaa390677c6d780f.wcvllevkmvpg2fd.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/jsuarez/Code/junosuarez/aoc2019/day1/target/debug/deps" "-L" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd-ec578e0d01ad5d6e.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-5412e5af11009a97.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-03db0718fbd4a443.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-8df90fdde44531fa.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-080b75c76cf389d3.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-954947c96c071ed1.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-9a1775bac6aabe20.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libunwind-71147793b4cdc412.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-9fc81eecc6136c9a.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liblibc-4b64712313317864.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liballoc-1bcd644d1289b2fb.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-16c65b3b16ee989d.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcore-7dd67903be10326a.rlib" "/Users/jsuarez/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-b5923fb6eca9603a.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
  = note: xcrun: error: invalid active developer path (/Library/Developer/CommandLineTools), missing xcrun at: /Library/Developer/CommandLineTools/usr/bin/xcrun

error: aborting due to previous error

error: could not compile `day1`.

To learn more, run the command again with --verbose.

</details>

the note at the bototm looks helpful: `invalid active developer path (/Library/Developer/CommandLineTools), missing xcrun at: /Library/Developer/CommandLineTools/usr/bin/xcrun`

I'm running this on my home laptop, which has a bunch of mishmasshed develoepr tools, but apparently not the xcode cli tools. i recently updated to macOs catalina, so maybe that's it.

i can install this easily enough, but it would have been nice if the rust installer had warned me about this, or if the compiler had given me a note about how to install them:

`xcode-select --install`

this took a long time, about 5 minutes, and i'm not sure why. computers are complicated at the end of the 2010s.

now i can successfully do `cargo run` and it prints some information and then, finally, the program output: Hello world!

i'm going to be running me code a lot during this, i think- can i get it to print a plain stdout without the build info?

i try `cargo help run` and behold, it works (thanks, modern cli subcommand conventions!) and i might have a flag `--q -q No output printed to stdout`

and sure enough `cargo run -q` gives me what i want: only the program output. also, since the program was already compiled this time, it was super fast. `cargo run` (without the flag) confirms that it skipped recompilation. this is a nice touch. modern toolchains are nice.

let's see what it wrote to disk:

- i've got a `Cargo.lock` file (which I assume corresponds to a package.lock or yarn.lock file - I'll set aside package management for now and dig more into Cargo packages aka "crates" later. I'm a bit miffed that these files are capitalized, but I guess it doesn't matter.)
- I've got a `/target` directory alongside my `/src` directory, with a `/debug` subdir. i noticed "debug" in the output from the earlier `cargo run` command, so presumably it's a binary with some extra debugger hooks, and presumably there's ways to target an optimized or production build.
- inside the `/target/debug` there's a whole bunch that I didn't expect. I'll ignore trying to figure out what everything is, but it looks like i've got an executable binary at `/target/debug/day1`. i'll add `target` to my gitignore for this repo. maybe there are better rust conventions- i'll learn them later.

sure enough, running `target/debug/day1` gives me my program output.

okay, this was exciting! let's crack the first aoc challenge. first, though, i'm going to rename day1 to day01 so i can get proper lexicographical sorting past day 9. since i haven't really done anything, i'm just going to `rm -rf day1` and `cargo new day01`.

now, day 1: https://adventofcode.com/2019/day/1

### Part 1

this is implementing a simple math formula. i know no rust. but, i've heard how good it's compiler error messages are, and that it's somewhat C-like. so, naively, i implement it basically like javascript, taking a cue from the scaffolded hello world from `cargo new`:

```rust
fn main() {
    println!(getRequiredFuel(12));
}

fn getRequiredFuel(mass) {
  return Math.floor(mass / 3) - 2
}
```

and i immediately get a ton of errors! but let's see if i can work through from the compiler and teach myself what it should be.

first:

```
error: expected one of `:`, `@`, or `|`, found `)`
 --> src/main.rs:5:24
  |
5 | fn getRequiredFuel(mass) {
  |                        ^ expected one of `:`, `@`, or `|` here
  |
  = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this was a parameter name, give it a type
  |
5 | fn getRequiredFuel(mass: TypeName) {
  |                    ^^^^^^^^^^^^^^
help: if this is a type, explicitly ignore the parameter name
  |
5 | fn getRequiredFuel(_: mass) {
  |                    ^^^^^^^
```

this is astonishingly good. I don't know what any of the types are called, but let's try `int`. that worked! on to the next error.

```
error: format argument must be a string literal
 --> src/main.rs:2:14
  |
2 |     println!(getRequiredFuel(12));
  |              ^^^^^^^^^^^^^^^^^^^
help: you might be missing a string literal to format with
  |
2 |     println!("{}", getRequiredFuel(12));
  |              ^^^^^
```

okay, looks like I can use the string format version they provide. that takes care of that second error.

hmm, looks like `int` didn't really work, it just got rid of the verbose untyped parameter error. now i've got

```
error[E0412]: cannot find type `int` in this scope
 --> src/main.rs:5:26
  |
5 | fn getRequiredFuel(mass: int) {
  |                          ^^^ not found in this scope
```

maybe `Int`? no. `integer`? no. `Integer`? no. hmmm, a long shot, but `number`/`Number`? no. ugh. I'll have to look at _some_ docs, I guess. This should be easy though. I search `rust number` and get linked to https://doc.rust-lang.org/book/ch03-02-data-types.html which has a handy table. Ah, rust has very specific numeric types. I'll go with a signed 32-bit integer, `i32` which would correspond to a java `int`, I think. that takes care of the paremeter type error.

now i've got left my javascripty Math.floor. i didn't really expect it to just work, but it was my first guess. i get

```
error[E0425]: cannot find value `Math` in this scope
 --> src/main.rs:6:10
  |
6 |   return Math.floor(mass / 3) - 2
  |          ^^^^ not found in this scope

```

I'm gonna have to go to docs, so i search `rust math floor` and get https://docs.rs/libmath/0.1.4/math/round/fn.floor.html

ah, i've got to import it with `use math::round;` and i also notice i've got to be explicit about the "scale" of the number type i want to rount it to, in this case i think i'll try i32. i guessed wrong from my glance at the docs! the scale parameter is a number of decimal places, not the data type. i don't want decimal places, so 0.

my code now looks like

```rust
use math::round;

fn main() {
    println!("{}", getRequiredFuel(12));
}

fn getRequiredFuel(mass: i32) {
  return round::floor(mass / 3, 0) - 2;
}
```

but my import isn't working:

```
error[E0432]: unresolved import `math`
 --> src/main.rs:1:5
  |
1 | use math::round;
  |     ^^^^ use of undeclared type or module `math`
```

hmm, I copied `use math::round;` right from the docs. one thing i notice is the `::` notation is different from what i'm used to in other languages (which would use a . or a -> to traverse a namespace). clicking around the docs i notie the page i pulled up mentions "crate". is it possible that these math functions aren't builtin to the language, and that i need to specify a dependency? sure enough clicking around gets me to https://github.com/0x022b/libmath-rs which sure looks like an open source third party package. hmm, let's verify that there really isn't a builtin. this feels like an extreme level of modularity- is this typical in rust? lower in my search results for `rust math floor` i find https://doc.rust-lang.org/std/primitive.f64.html which looks promising. the number types (or 64-bit floats, anyway) have a built-in floor method. let me try:

```rust
fn getRequiredFuel(mass: i32) {
  return (mass / 3).floor() - 2;
}
```

and i get

```
error[E0599]: no method named `floor` found for type `i32` in the current scope
 --> src/main.rs:6:21
  |
6 |   return (mass / 3).floor() - 2;
  |                     ^^^^^ method not found in `i32`
```

well. what's the behavior of the plain `/` function? after some experimentation, it looks like it rounds. this is intuition i should have from other languages (C, Java), but for some reason i thought it would do what-you-learned-in-middle-school rounding (aka Half Up). silly. specifically, the behavior i saw experimentally was rounding down towards 0, eg 99/100 => 0, -99/100 => 0.

so i can remove the function call and use the basic `/` operator. i add a comment explicating the rounding behavior, giving:

```rust
fn getRequiredFuel(mass: i32) {
  // we depend on i32 rounding, which rounds the division down toward 0
  return (mass / 3) - 2;
}
```

progress! now i've got:

```
error[E0308]: mismatched types
 --> src/main.rs:7:10
  |
5 | fn getRequiredFuel(mass: i32) {
  |                               - help: try adding a return type: `-> i32`
6 |   // we depend on i32 rounding, which rounds the division down toward 0
7 |   return (mass / 3) - 2;
  |          ^^^^^^^^^^^^^^ expected (), found i32
  |
  = note: expected type `()`
             found type `i32`
```

which helpfully tells me i need to document my return type. the help hint is spot on.

now my code compiles and gives the right result! 🎉

it looks like:

```rust
fn main() {
    println!("{}", getRequiredFuel(12));
}

fn getRequiredFuel(mass: i32) -> i32 {
  // we depend on i32 rounding, which rounds the division down toward 0
  return (mass / 3) - 2;
}
```

no more errors, but i get a helpful style warning:

```
warning: function `getRequiredFuel` should have a snake case name
 --> src/main.rs:5:4
  |
5 | fn getRequiredFuel(mass: i32) -> i32 {
  |    ^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `get_required_fuel`
  |
  = note: `#[warn(non_snake_case)]` on by default

```

we're not in Java(Script) anymore! i've heard there's a rustfmt tool, let's see if that can fix it for me. i run it from my shell, and it just hangs for a while. i ctrl-c it and run `rustfmt --help`. i rerun it as `rustfmt src/main.rs` and it exits without printing anything. i try `rustfmt --check src/main.rs`, which likewise exits (code 0) without printing. my `src/main.rs` file is unchanged. looks like the printer won't fix this style warning for me.

i manually change it to:

```rust
fn main() {
    println!("{}", get_required_fuel(12));
}

fn get_required_fuel(mass: i32) -> i32 {
    // we depend on i32 rounding, which rounds the division down toward 0
    return (mass / 3) - 2;
}
```

which compiles cleanly and prints my output, `2`.

i try it with the remaining examples form the puzzel. all my results are right, so now we can solve the puzzel. sometime soon i'll have to figure out tests in rust.

i've never done an aoc before, and i didnt' really read before starting, so now i realize i've got a big input file. i hate copy/pasting code, so i guess i need to figure out how to read from stdin. i noticed one of the headings form the rust book docs: https://doc.rust-lang.org/book/ch12-00-an-io-project.html . time to read, i guess. it says it'll reference earlier concepts, but who's got time for that? let's see what I can clean.

ah, reading command line args! https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#reading-the-argument-values let's modify the program to read the number as input and then print it. i'll need to parse it from a string to an int.

so `env::args()` will give me an "iterator" of the arguments. with the RLS extension in vscode, i get intellisense in its members, so i try `env::args().next`, in full `let x: string = env::args().next();`.

looks like i'm running into some more typing things! .next gives me an `Option<String>`. This looks familiar from Java.

RLS is great, it's giving me lots of helpful inline documentation in my editor (vscode). I fumble my way though the String.parse docs. There's a syntax called a "turbofish" and apparently type inference.

i get an error. i realize that calling `env::args().next` is giving me arg0, my binary path, instead of what i'm passing in. instead of being super hacky, let me read about converting the iterator to a collection, like the tutorial was trying to help me with: `let args: Vec<String> = env::args().collect();`.

after some fumbling, i have:

```rust
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  let arg1 = args.get(1).expect("Missing input");
  let input: i32 = arg1.parse().unwrap();

  println!("{}", get_required_fuel(input));
}

fn get_required_fuel(mass: i32) -> i32 {
  // we depend on i32 rounding, which rounds the division down toward 0
  return (mass / 3) - 2;
}
```

which works. for now, i'm going to use some shell script to get through the first puzzel's input and
not worry about file reading, nor control flow or loops, for now.

i've got this shell script:

```sh
#! /bin/bash

INPUT=$(cat input)

SUM=0

for MASS in $INPUT; do
  FUEL_FOR_MODULE=$(target/debug/day01 $MASS)
  SUM=$(expr $FUEL_FOR_MODULE + $SUM)
done

echo $SUM
```

which gives me an answer of 3406527. and I've got the first start! 🌟

### Part 2

We've got recusion! Unfortunately, I think this means I can't avoid reading all of the values into my rust program any longer, but that will have to wait for my next session.
