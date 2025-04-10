---
title = "building my website!"
summary = "going over the features in my website, and how i built them"
date = 2025-04-10
id = "celestials-closet"
---

## motivation

i went into this project with a few things in mind:

- if something looks fun to write, i'll write it from scratch (ex. the scrapped custom data format). it helps me learn, and it makes things more enjoyable overall.
- keep things simple, small, efficient, and fast.
- also, i wanted to see if i could pack the entire website into a single binary

## embedding everything into static binary

using rust's [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
i was able to pack all of the assets and posts into 1 binary.
build scripts definitely are not intended for this.. but it works!

anyways, i packed everything into 1 binary because:

- none of the assets or posts will change often
- 1 binary is easier to handle than 20 assets
- filesystem calls are fallible, and more expensive than ram
- it was fun :3

now that i've implemented it, i dont think i used build scripts for what they were made for. instead of abusing build scripts like this:

```rust
fn hashmap(name: &str, map_sig: &str, entries: impl Iterator<Item = impl Display>) -> String {
    format!(
        "use std::{{sync::LazyLock, collections::HashMap}};

        static {}: LazyLock<HashMap<{}>> = LazyLock::new(|| HashMap::from([
            {}
        ]));",
        name,
        map_sig,
        entries.map(|v| v.to_string()).collect::<Vec<_>>().join(",\n")
    )
}
```

i probably should've used proc macros.
i *did* make sure to string escape the values with [proc_macro2::Literal](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Literal.html), so at least it won't break (hopefully).

anyways, i did the post processing, asset packing, and the template rendering (whenever i get to it) inside the build script,
and then the final .rs files get dumped into the website via `include!(concat!(env!("OUT_DIR"), "blah.rs"))`

## the markdown pipeline

my pipeline for markdown rendering is pretty simple:

1. transform the markdown into `impl Iterator<Item = Event>` via [pulldown-cmark](https://crates.io/crates/pulldown-cmark)
1. do some processing on the iterator, which allows for stuff like:
   - syntax highlighting
   - self-anchoring headers
   - table of contents
   - and basically anything else that i can do with an `impl Iterator<Item = Event>`
1. transform the iterator into HTML

the logic for the post parsing is in [build/posts.rs](https://github.com/CelestialCrafter/celestials-closet/blob/master/build/posts.rs),
except for the highlighting code, which lives in [build/highlighting.rs](https://github.com/CelestialCrafter/celestials-closet/blob/master/build/highlighting.rs)

### syntax highlighting ft. treesitter

the easiest way i could've done this was to just include a javascript library like [highlight.js](https://highlightjs.org/) or [Prism.js](https://prismjs.com/), but i want to keep javascript to a minimum.

i considered [syntect](https://crates.io/crates/syntect), but didnt use it because
syntect uses sublime syntax definitions, which look excruciating to write, and i dont use sublime text.

since i use neovim as my editor, i considered (and ended up using) [tree-sitter](https://tree-sitter.github.io/) because:

- from using it in neovim and helix, it's *really* pleasant
- theres a big [list of parsers](https://github.com/tree-sitter/tree-sitter/wiki/List-of-parsers), so any language i want to use should work.
- tree-sitter's api seems alot simpler to use than syntect's

the actual crate i ended up using is [tree-sitter-highlight](https://crates.io/crates/tree-sitter-highlight)
i got the highlight names from running `:help treesitter-highlight-groups` in neovim,
and adapting the groups to my usecase.

after that, we can iterate over the markdown parser

the code scans for code blocks, stores the language, finds the inner text blocks, and injects colored html from:

```rust
let highlights = highlighter.highlight(config, code.as_bytes(), None, |_| None)?;

let mut html = String::with_capacity(code.len());

for event in highlights.map(|e| {
    // if something goes wrong, print a warning and bail out of the current highlight
    e.unwrap_or_else(|err| {
        println!("cargo:warning=highlight error: {}", err);
        HighlightEvent::HighlightEnd
    })
}) {
    match event {
        // actual code
        HighlightEvent::Source { start, end } => escape_html(&mut html, &code[start..end]),
        // start highlighting a section
        HighlightEvent::HighlightStart(highlight) => {
            write!(html, "<span class=\"{}\"/>", HIGHLIGHT_NAMES[highlight.0])
        }
        // end the section's highlight
        HighlightEvent::HighlightEnd => write!(html, "</span>"),
    }
    .unwrap();
}

Ok(html)
```

this iterates over the highlights, and spits out the colored source code.

## custom data format

i originally hand rolled a comments system with a custom binary format...
and then scrapped it, because opening up an unrestricted comment box to the entire internet sounds like an awful idea.

here's the data format's layout:

```
1 byte header
    - ip type = bit 8 (0 for ipv4, and 1 for ipv6)
    - name length = bit 1 to 7 (0 for no message)

if ipv4:
    ip = read out 4 bytes
if ipv6:
    ip = read out 16 bytes

if name length > 0:
    content length = read out 1 byte
    name = read out <name length> bytes
    content = read out <content length> bytes
```

of course, i forgot to add a few version bits in the header
here's a sample packet, binary dumped w/ `xxd -b`:

```
00000000: 00001001 01111111 00000000 00000000 00000001 00001101  ......
00000006: 01100011 01100101 01101100 01100101 01110011 01110100  celest
0000000c: 01101001 01100001 01101100 01110100 01100101 01110011  ialtes
00000012: 01110100 00100000 01101101 01100101 01110011 01110011  t mess
00000018: 01100001 01100111 01100101 00100001                    age!
```

sample packet decoded:

```
00001001 -> header
    - bit 8 (ip type): ip type = ipv4
    - bit 1-7: name length = 9
01111111 00000000 00000000 00000001 -> ip = 127.0.0.1
00001101 -> content length = 13
the rest -> name, and then content
```

[source code](https://github.com/CelestialCrafter/celestials-closet/blob/c45cc37ee2e688be766d00bec4ec0621435fc036/src/database.rs)

## rendering

i went with server-side rendering (as you could probably guess), with askama as my templating engine.
i'm not happy with askama because:

- takes a while to compile templates
- a bunch of features i don't need, like:
  - filters
  - inheritance
  - custom syntax for control structures
  - runtime template variables
  - ...and a bunch of other stuff

i also don't like jinja's templating syntax, reminds me of python. (yuck, might write about it in a future post)
i looked at some other templating libraries, but most of them either:

- were too complicated
- had weird macro dsl's
- had too many features

### custom templating engine

i'm planning on writing my own templating engine, but i havent implemented it yet.
the general idea is:

each section (page, styles, metadata, etc) is a "node", which can be nested into other nodes.
at compiletime, build a [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph) of all the nodes, and compile from the leaves up to the root.
this gives me declarative & simple templating, without any
if i want dynamic logic, i can just run it at compile time, and insert it into the template.
pure rust, no macros, dsls, or features i dont want or need. just string interpolation.

<small>i would make a visual, but i just dont want to.. sorry!</small>

## asset optimization

<!-- @TODO show final binary size, explain why webp, explain dynamic resizing, explain how the recommended image size is calculated from lighthouse -->

## routing

for routing, i used [warp](https://crates.io/crates/warp).
it's super simple, fast, and has the perfect amount of features.

instead of the classic routing done in things like [actix](https://actix.rs/) or [express](https://expressjs.com/),
warp represents all routing and transformations as data, within the rust type system.

its main abstraction is the [Filter](https://docs.rs/warp/latest/warp/trait.Filter.html), which is
a composable piece of logic that takes in http requests,
and can output either a value (that gets passed down),
or a rejection (which means it doesn't match, and to let a different filter handle the requests)

and since filters are:
- represented as a type
- composable (via `.and()`, `.or()`, `.map()`, etc)
your entire routing tree is typed!

<details>
<summary>what rustc sees my website as</summary>

```rust
compression::internal::WithCompression<impl (Fn(compression::internal::CompressionProps) -> Response<Body>) + std::marker::Copy, warp::filter::recover::Recover<warp::filter::or::Or<warp::filter::or::Or<BoxedFilter<(impl Reply,)>, warp::filter::or::Or<warp::filter::or::Or<warp::filter::and::And<warp::filter::untuple_one::UntupleOne<warp::filter::map::Map<warp::filter::and::And<impl warp::Filter + warp::filter::FilterBase<Extract = (), Error = Infallible> + std::marker::Copy, impl warp::Filter + warp::filter::FilterBase<Extract = (Option<std::net::SocketAddr>,), Error = Infallible> + std::marker::Copy>, {closure@src/routes.rs:17:14: 17:45}>>, warp::filter::then::Then<impl warp::Filter + warp::filter::FilterBase<Extract = (), Error = Rejection> + std::marker::Copy, fn() -> impl Future<Output = impl Reply> {index::page}>>, warp::filter::then::Then<Exact<warp::path::internal::Opaque<&str>>, fn() -> impl Future<Output = impl Reply> {projects::page}>>, warp::filter::and::And<Exact<warp::path::internal::Opaque<&str>>, warp::filter::or::Or<warp::filter::then::Then<impl warp::Filter + warp::filter::FilterBase<Extract = (), Error = Rejection> + std::marker::Copy, fn() -> impl Future<Output = impl Reply> {listing}>, warp::filter::and_then::AndThen<impl warp::Filter + warp::filter::FilterBase<Extract = (String,), Error = Rejection> + std::marker::Copy, fn(String) -> impl Future<Output = Result<impl Reply, Rejection>> {posts::post}>>>>>, warp::filter::then::Then<Exact<warp::path::internal::Opaque<&str>>, fn() -> impl Future<Output = impl Reply> {personal::page}>>, fn(Rejection) -> impl Future<Output = Result<impl Reply, Rejection>> {handle}>>
```

</details>

using types for routing has alot of benefits, especially within the context of the rust type system.
this approach lets you get compile time checking for routes,
and there's also no room for invalid states. every request either pattern matches into the site, or gets rejected.

## deployment

since everything is packed into single binary, the site is super simple to get running.
by default (assuming a `x86_64-unknown-linux-gnu` target), rust dynamically links to glibc,
but i can produce a statically linked binary by changing the build target to [musl](https://www.musl-libc.org/).
statically linking the binary solves a few portability concerns:
\- no pkgconfig, extra libraries to install, or versioning system libraries
\- no dealing with patchelf from binaries built on nix
with the tradeoff of bigger binaries..
but im already packing like 20 images inside of the binary,
so statically linking a few more libraries shouldnt make *too* much of a difference.

anyways, since i use nix as the build system, i can easily cross-compile to aarch64 and x86_64:

```nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    {
      packages = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          program = pkgs.rustPlatform.buildRustPackage {
            pname = "celestials-closet";
            version = "0.1.0";
            src = nixpkgs.lib.cleanSource ./.;
            cargoHash = "sha256-RMvbp9wh7GPhy4EgZDsfl+7Wn3isWwZRIr7TRmjEFEM=";
            useFetchCargoVendor = true;
          };
        in
        {
          inherit default;
        }
      );
    };
}
```

if you want more information on my weird server/deployment pipeline, that'll be in my next post.
thanks for reading! \<3

## @TODO

- table of contents
- publish date + revision date
- display lang/origin in codeblocks
- 404 page
- gallery
- custom templating engine
