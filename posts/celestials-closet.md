---
title = "building my website!"
summary = "going over my website's features, and how i built them"
date = 2025-04-10
id = "celestials-closet"
---

## motivation

i went into this project with a few things in mind:

- if something seems fun to write, i'll write it from scratch. it helps me learn, and it makes things more enjoyable overall.
- keep it simple, small, efficient, and fast.
- also, i wanted to see if i could pack the entire website into a single binary

## routing

for routing, i used [warp](https://crates.io/crates/warp).
it's super simple, fast, and has the perfect amount of features.

instead of the classic routing done in things like [actix](https://actix.rs/) or [express](https://expressjs.com/),
warp represents all routing and transformations as data, within the rust type system.

its main abstraction is the [Filter](https://docs.rs/warp/latest/warp/trait.Filter.html), which is
a composable piece of logic that takes in http requests,
and can output either a value (that gets passed down)
or a rejection (which means it doesn't match, and lets a different filter handle the request)

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

using types for routing has a lot of benefits, especially within the context of the rust type system.
it lets you get compile-time checking for routes, type checked refactors,
and there's also no room for invalid states.
every request either pattern matches into the site, or gets rejected.

## templating

i went with askama as my templating engine, but i'm also not happy with askama.

it takes a while to compile templates,
its syntax is based on jinja (derogatory; reminds me of python)
and has a bunch of features i don't need, like:

- filters
- inheritance
- custom syntax for control structures
- runtime template variables
- ...and a bunch of other stuff

i looked at some other templating libraries, but most of them either:

- were too complicated
- had weird macro dsl's
- had too many features

### custom templating engine

i'm planning on writing my own templating engine, but i haven't implemented it yet.
the general idea is:

each section (page content, styles, metadata, etc) is a "node", which can be nested into other nodes.
at compile-time, build a [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph) of all the nodes, and compile from the leaves up to the root.
this gives me declarative & simple templating, without any extra stuff.
if i want dynamic logic, i can just run it at compile time and insert it into the template.
pure rust, no macros, dsls, or features i don't want or need. just string interpolation.

<small>i would make a visual, but i just don't want to.. sorry!</small>

## the markdown pipeline

my pipeline for markdown rendering is pretty simple:

1. transform the markdown into `impl Iterator<Item = Event>` (w/ [pulldown-cmark](https://crates.io/crates/pulldown-cmark))
1. do some processing on the iterator, which allows for stuff like:
   - syntax highlighting
   - self-anchoring headers
   - table of contents
   - and basically anything else that i can do with an `impl Iterator<Item = Event>`
1. transform the iterator into HTML

the logic for parsing posts is in [build/posts.rs](https://github.com/CelestialCrafter/celestials-closet/blob/master/build/posts.rs),
except for the highlighting code that lives in [build/highlighting.rs](https://github.com/CelestialCrafter/celestials-closet/blob/master/build/highlighting.rs)

### syntax highlighting ft. treesitter

the easiest way would've been to just include a javascript library like [highlight.js](https://highlightjs.org/) or [Prism.js](https://prismjs.com/), but i want to keep javascript to a minimum.

i considered [syntect](https://crates.io/crates/syntect), but didn't use it because
syntect uses sublime syntax definitions, which look excruciating to write, and i don't use sublime text.

since i use neovim as my editor, i considered (and ended up using) [tree-sitter](https://tree-sitter.github.io/) because:

- from using it in neovim and helix, it's *really* pleasant
- there's a big [list of parsers](https://github.com/tree-sitter/tree-sitter/wiki/List-of-parsers), so any language i want to use should work.
- tree-sitter's api seems a lot simpler to use than syntect's
- it's *really* fast

the actual crate i ended up using is [tree-sitter-highlight](https://crates.io/crates/tree-sitter-highlight).
i got the highlight names from running `:help treesitter-highlight-groups` in neovim and adapting the groups to my use case.

now that we have our highlight groups set up,
we can start highlighting, iterate over the highlights,
and generate html that gets fed back into the markdown parser:

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
            write!(html, "<span class=\"{}\">", HIGHLIGHT_NAMES[highlight.0])
        }
        // end the section's highlight
        HighlightEvent::HighlightEnd => write!(html, "</span>"),
    }
    .unwrap();
}

Ok(html)
```

this iterates over the highlights, and spits out the colored source code.

<details>
<summary>sample highlighting output</summary>

```html
<pre><code class="language-rust"><span class="keyword">fn</span> <span class="function">main</span><span class="punctuation">(</span><span class="punctuation">)</span> <span class="punctuation">{</span>
    <span class="function">println</span><span class="function">!</span><span class="punctuation">(</span><span class="string">&quot;hello world! &lt;3&quot;</span><span class="punctuation">)</span><span class="punctuation">;</span>
<span class="punctuation">}</span></code></pre>
```

</details>

## embedding everything into the binary

using rust's [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
i was able to pack all of the assets and posts into one binary.

i did this because:

- none of the assets or posts change often
- a single binary is easier than managing 15+ assets
- filesystem calls are fallible and more expensive than ram
- it was fun :3

anyways, i used the build script to do post processing and asset handling,
then the final .rs files get dumped into the website via `include!(concat!(env!("OUT_DIR"), "file.rs"))`

if i did (i may) rewrite the build pipeline in the future,
i'd keep the build scripts for post creation/asset handling,
but use [include_bytes](https://doc.rust-lang.org/std/macro.include_bytes.html) for the data,
and then using [proc macros](https://doc.rust-lang.org/reference/procedural-macros.html),
i can include a hashmap of all posts and assets.

### asset optimization

since i'm packing all of the assets into the binary,
i need to optimize them a bit before packing them in.

i wasn't very aggressive on optimization, i only did:

- conversion to lossless [webp](https://developers.google.com/speed/webp)
- resizing images to the displayed size (excluding project preview images)

i used to handle the image optimization manually,
but it was time consuming and destructive to the original assets

i needed different transforms on some assets,
so i opted for using optimization levels (unchanged, webp, webp + resize).

```
ado.jpg.opt-2              mimosa-confessions.jpg.opt-2
beabadoobee.jpg.opt-2      needy-streamer-overload.png.opt-2
bocchi-the-rock.jpg.opt-2  nier-automata.jpg.opt-2
dotfiles-1.webp.opt-1      niki.jpg.opt-2
dotfiles-2.webp.opt-1      oshi-no-ko.webp.opt-2
ed25519.txt.opt-0          persona-5-royal.jpg.opt-2
frieren.webp.opt-2         profile.jpg.opt-2
girls-band-cry.jpg.opt-2   witch-hat-atlier.jpg.opt-2
inabakumori.jpg.opt-2      yorushika.png.opt-2
iyowa.jpg.opt-2            zutomayo.jpg.opt-2
lycoris-recoil.jpg.opt-2
```

from there, i can simply parse the optimization levels in my build pipeline,
and transform the assets as needed:

```rust
match opt_level {
    0 => {}
    1 | 2 => {
        let mut img = ImageReader::new(Cursor::new(&buffer))
            .with_guessed_format()?
            .decode()?;
        if opt_level == 2 {
            img = img.resize(IMAGE_SIZE, IMAGE_SIZE, FilterType::Lanczos3);
        }

        buffer.clear();
        img.write_with_encoder(WebPEncoder::new_lossless(&mut buffer))?;

        // remove the .opt-<level> extension, then change the filename extension to webp
        path.set_extension("");
        path.set_extension("webp");
    }
    _ => return Err(eyre!("optimization level {opt_level} is unsupported")),
}
```

### how does it affect file size?

the size of the binary is 5.8MB, and if we want to find out how much of that is assets,
we can look through the binary itself!

from the [specification](https://developers.google.com/speed/webp/docs/riff_container#webp_file_header),
the file size for WebP is sandwiched in between `RIFF` and `WEBP` as a little-endian u32.
<small> this does mean we will only be searching for webp (ignoring non-image assets and markdown), but images are the biggest.</small>

since we will be searching through a hexdump of the binary,
we'll need to make a script that converts big-endian hex to little-endian decimal:

```lua
-- stolen from https://stackoverflow.com/a/72784448
local function reverse(table)
	for i = 1, #table // 2, 1 do
		table[i], table[#table - i + 1] = table[#table - i + 1], table[i]
	end

	return table
end

for hex in io.lines() do
	-- split bytes into pairs
	local pairs = {}
	for i = 1, #hex, 2 do
		table.insert(pairs, hex:sub(i, i + 1))
	end

	-- convert to little endian decimal, and print it out
	print(tonumber(table.concat(reverse(pairs), ""), 16))
end
```

now, we can use a little shell scripting to get our result:

```fish
# hexdump of the binary
xxd -ps target/release/celestials-closet | \
    # search for webp files, match the file size
    # RIFF = 52 49 45 46
    # WEBP = 57 45 42 50
    rg --only-matching --replace '$1' '52494646(.{8})57454250' | \
    # convert from big-endian hexadecimal -> little-endian deicmal
    lua convert.lua | \
    # sum all the sizes up
    awk '{sum+=$1} END {print sum}'
```

and at the time of writing, this outputs `888632`, or 888KB out of 5912KB (or ~15%).

## custom data format

i put this last because it doesn't exist anymore,
but i originally created a comment/views system with a hand-rolled binary format..
and then scrapped it because opening up an unrestricted comment box to the entire internet sounds like an awful idea.

here's the layout of the data format:

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

of course, i forgot to add a few version bits in the header.
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

## deployment

by default (assuming a `x86_64-unknown-linux-gnu` target), rust dynamically links to glibc.
*but* i can produce a statically linked binary by changing the build target to [musl](https://www.musl-libc.org/).

statically linking the binary solves a few portability concerns:

- no pkg-config, extra libraries to install, or dealing with different platforms having differently versioned libraries
- no dealing with patchelf from binaries built on nix

with the trade-off being (slightly) bigger binaries (and some other things, which i won't go into here).
but i'm already packing like 20 images inside of the binary,
so statically linking a few more libraries shouldn't make *too* much of a difference.

anyways, since i use nix as my build system, i can easily cross-compile to aarch64 and x86_64:

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

thanks for reading! \<3

## @TODO

things i still need to do.

this is mainly a reference for future me, but read through it if you want!

### table of contents

i'm already able to inject into the markdown processing pipeline,
so this shouldn't be too hard.

### dates and change ids

for dates, i want:

- publish date/publish change id
- latest revision date/latest revision change id

when i implemented this in the past, there were a few issues:

if i use the [jujutsu](https://jj-vcs.github.io/jj/) library, the compile times are insufferably long.
and if i use the jj cli, parsing the output feels really wrong and unstable,
even though it's probably not.

### lang/origin in codeblocks

again, more markdown parsing stuff.
i'd probably implement this via doing some custom parsing in the language codeblocks.

### misc

- 404 page
- [custom templating engine](#custom-templating-engine)
