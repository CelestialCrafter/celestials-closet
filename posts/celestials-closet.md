---
title = "Building my Website!"
summary = "Going over my website's features, and how I built them."
date = 2025-04-10
id = "celestials-closet"
---

## Motivation

I went into this project with a few things in mind:

- If something seems fun to write, I'll write it from scratch.
  It helps me learn, and makes things more enjoyable overall
- Keep the site simple, small, efficient, and fast
- I also wanted to see if I could pack the entire website into a single binary

## Routing

For routing, I used [warp](https://crates.io/crates/warp).
It's simple, fast, and has the perfect amount of features.

Instead of the classic routing done in things like [actix](https://actix.rs/) or [express](https://expressjs.com/),
warp represents all routing and transformations as data, within the Rust type system.

It's main abstraction is the
[Filter](https://docs.rs/warp/latest/warp/trait.Filter.html), which is a
composable piece of logic that takes in http requests, and can output either
a value (that gets passed down) or a rejection (which lets a different filter
handle the request).

Since filters are both represented as a type and composable (via `.and()`,
`.or()`, `.map()`, etc), the entire routing tree is typed!

<details>
<summary>What `rustc` sees my website as</summary>

```rust
compression::internal::WithCompression<impl (Fn(compression::internal::CompressionProps) -> Response<Body>) + std::marker::Copy, warp::filter::recover::Recover<warp::filter::or::Or<warp::filter::or::Or<BoxedFilter<(impl Reply,)>, warp::filter::or::Or<warp::filter::or::Or<warp::filter::and::And<warp::filter::untuple_one::UntupleOne<warp::filter::map::Map<warp::filter::and::And<impl warp::Filter + warp::filter::FilterBase<Extract = (), Error = Infallible> + std::marker::Copy, impl warp::Filter + warp::filter::FilterBase<Extract = (Option<std::net::SocketAddr>,), Error = Infallible> + std::marker::Copy>, {closure@src/routes.rs:17:14: 17:45}>>, warp::filter::then::Then<impl warp::Filter + warp::filter::FilterBase<Extract = (), Error = Rejection> + std::marker::Copy, fn() -> impl Future<Output = impl Reply> {index::page}>>, warp::filter::then::Then<Exact<warp::path::internal::Opaque<&str>>, fn() -> impl Future<Output = impl Reply> {projects::page}>>, warp::filter::and::And<Exact<warp::path::internal::Opaque<&str>>, warp::filter::or::Or<warp::filter::then::Then<impl warp::Filter + warp::filter::FilterBase<Extract = (), Error = Rejection> + std::marker::Copy, fn() -> impl Future<Output = impl Reply> {listing}>, warp::filter::and_then::AndThen<impl warp::Filter + warp::filter::FilterBase<Extract = (String,), Error = Rejection> + std::marker::Copy, fn(String) -> impl Future<Output = Result<impl Reply, Rejection>> {posts::post}>>>>>, warp::filter::then::Then<Exact<warp::path::internal::Opaque<&str>>, fn() -> impl Future<Output = impl Reply> {personal::page}>>, fn(Rejection) -> impl Future<Output = Result<impl Reply, Rejection>> {handle}>>
```

</details>

Using types for routing has a lot of benefits, especially within the context
of the rust type system. It lets you get compile-time checking for routes, type
checked refactors, and there's also no room for invalid states, every request
either pattern matches into the site, or gets rejected.

## Templating

I went with askama as my templating engine, but i'm not happy with it because:

- It takes a while to compile templates,
- Its syntax is based on jinja (derogatory; reminds me of python)
- It has a bunch of features I don't want or need, such as:
  - Filters
  - Inheritance
  - Custom syntax for control structures
  - Runtime template variables
  - ...and a bunch of other stuff

I looked at some other templating libraries, but most of them were either too
complicated, had weird macro DSL's, or just had too many features.

### Custom templating engine

I'm planning on writing my own templating engine, but i haven't implemented it yet.
the general idea is:

Each section (page content, styles, metadata, etc) is a "node", which can be nested into other nodes.
At compile-time, it will build a [DAG](https://en.wikipedia.org/wiki/Directed_acyclic_graph) of all the nodes,
and compile from the leaves up to the root.

This gives me declarative & simple templating, without any extra features.
If I want dynamic logic, I can just run it at compile time and insert it into the template.
It can easily be implemented as pure Rust with simple string interpolation,
without any macros, DSL's, or features I don't want or need.

<small>I would make a visual, but I don't want to.. sorry!</small>

## Markdown pipeline

My pipeline for markdown rendering is pretty simple:

1. Transform the markdown into `impl Iterator<Item = Event>` (w/ [pulldown-cmark](https://crates.io/crates/pulldown-cmark)).
1. Do some processing on the iterator, which allows for stuff like:
   - Syntax highlighting
   - Self-anchoring headers
   - Table of contents
   - And basically anything else that I can do with an `impl Iterator<Item = Event>`
1. Transform the iterator into HTML

The logic for parsing posts is in [build/posts.rs](https://github.com/CelestialCrafter/celestials-closet/blob/master/build/posts.rs),
except for the highlighting code, which lives in [build/highlighting.rs](https://github.com/CelestialCrafter/celestials-closet/blob/master/build/highlighting.rs)

### Syntax highlighting ft. Treesitter

The easiest way to do syntax highlighting would've been to just include a JavaScript library like
[highlight.js](https://highlightjs.org/) or [Prism.js](https://prismjs.com/),
but I wanted to keep JavaScript to a minimum.

I considered [syntect](https://crates.io/crates/syntect), but didn't use it
because it uses sublime syntax definitions, which look excruciating to write,
and I don't use sublime text.

Since I use neovim as my editor, I considered (and ended up using) [tree-sitter](https://tree-sitter.github.io/) because:

- From using it in NeoVim and Helix, it's very fast and accurate
- There's a big [list of parsers](https://github.com/tree-sitter/tree-sitter/wiki/List-of-parsers), so any language I want to use should work.
- Tree-sitter's api seems a lot simpler to use than syntect's

The actual crate I ended up using is [tree-sitter-highlight](https://crates.io/crates/tree-sitter-highlight).
I got the highlight names from running `:help treesitter-highlight-groups` in NeoVim and adapting the groups to my use case.

Now that we have our highlight groups set up,
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

This iterates over the highlights, and outputs out the colored source code.

<details>
<summary>Sample highlighting output</summary>

```html
<pre><code class="language-rust"><span class="keyword">fn</span> <span class="function">main</span><span class="punctuation">(</span><span class="punctuation">)</span> <span class="punctuation">{</span>
    <span class="function">println</span><span class="function">!</span><span class="punctuation">(</span><span class="string">&quot;hello world! &lt;3&quot;</span><span class="punctuation">)</span><span class="punctuation">;</span>
<span class="punctuation">}</span></code></pre>
```

</details>

## Embedding everything into the binary

Using Rust's [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
I was able to pack all of the assets and posts into one binary.

I did this because:

- None of the assets or posts change often
- A single binary is easier than managing 15+ assets
- Filesystem calls are fallible and more expensive than ram
- It was fun :3

Anyways, I used the build script to do post processing and asset handling,
and then the final .rs files get dumped into the website via `include!(concat!(env!("OUT_DIR"), "file.rs"))`

If I do rewrite the build pipeline in the future,
I'd keep the build scripts for post creation/asset handling,
but use [include_bytes](https://doc.rust-lang.org/std/macro.include_bytes.html) for the data,
and then using [proc macros](https://doc.rust-lang.org/reference/procedural-macros.html),
I can include a hashmap of all posts and assets.

### Asset optimization

Since I'm packing all of the assets into the binary,
I need to optimize them a bit before packing them in.

I wasn't very aggressive with optimization, I only:

- Converted the images to lossless [WebP](https://developers.google.com/speed/webp)
- Resizing images to the displayed size (excluding project preview images)

I used to handle the image optimization manually, but it was time consuming
and destructive to the original assets, so I needed different transforms on
some assets.

I opted to define optimization levels (such as unchanged, webp, webp resize),
and then i can simply parse the optimization levels in my build pipeline, and
transform the assets as needed.

<details>
<summary>Example of optimization levels on assets</summary>

```
bocchi-the-rock.jpg.opt-2         kessoku-band.jpg.opt-2
celestial-ed25519.pub.opt-0       lycoris-recoil.jpg.opt-2
cosmic-princess-kaguya.jpg.opt-2  makeine.png.opt-2
deco-27.jpg.opt-2                 natori.jpg.opt-2
dotfiles-1.webp.opt-1             niki.jpg.opt-2
dotfiles-2.webp.opt-1             oshi-no-ko.webp.opt-2
frieren.jpg.opt-2                 profile.jpg.opt-2
girls-band-cry.jpg.opt-2          quicksand.woff2.opt-0
hikaru.webp.opt-2                 takopi.webp.opt-2
inabakumori.jpg.opt-2             witch-hat-atlier.jpg.opt-2
iyowa.jpg.opt-2                   yorushika.png.opt-2
jetbrains-mono.woff2.opt-0        zutomayo.jpg.opt-2
```

</details>

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

### How does it affect file size?

At the time of writing, the size of the binary is ~5.8MB, and if we want to find
out how much of that is assets, we can look through the binary itself.

From the [specification](https://developers.google.com/speed/webp/docs/riff_container#webp_file_header),
the file size for WebP is placed in between `RIFF` and `WEBP` as a little-endian u32.
<small>This does mean we will only be searching for webp (ignoring non-image
assets and markdown), but images are the biggest.</small>

Since we will be searching through a hexdump of the binary,
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

Now, we can use a little bit of shell scripting to get our result:

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

And at the time of writing, this outputs `888632`, or 888KB out of 5912KB (or ~15%).

## Deployment

By default (assuming a `x86_64-unknown-linux-gnu` target), Rust dynamically
*links to glibc. but* I can produce a statically linked binary by changing the
\*build target to [musl](https://www.musl-libc.org/).

Statically linking the binary solves a few portability concerns:

- No `pkg-config`, extra libraries to install, or dealing with different
  platforms having differently versioned libraries
- No dealing with `patchelf` from binaries built on NixOS

With the trade-off being (slightly) bigger binaries (and some other things, which I won't go into here).
but I'm already packing like 20 images inside of the binary,
so statically linking a few more libraries shouldn't make too much of a difference.

Anyways, since I use nix as my build system, I can easily cross-compile to `aarch64`:

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

## Custom data format

I put this last because it doesn't exist anymore, but I originally created a
comment/views system with a hand-rolled binary format, and then scrapped it
because opening up an unrestricted comment box sounds like a bad idea.

The layout of the data format is something similar to this:

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

And of course, I forgot to add a byte for the version in the header.

Here's a sample packet, binary dumped w/ `xxd -b`:

```xxd
00000000: 00001001 01111111 00000000 00000000 00000001 00001101  ......
00000006: 01100011 01100101 01101100 01100101 01110011 01110100  celest
0000000c: 01101001 01100001 01101100 01110100 01100101 01110011  ialtes
00000012: 01110100 00100000 01101101 01100101 01110011 01110011  t mess
00000018: 01100001 01100111 01100101 00100001                    age!
```

Sample packet decoded:

```
00001001 -> header
    - bit 8 (ip type): ip type = ipv4
    - bit 1-7: name length = 9
01111111 00000000 00000000 00000001 -> ip = 127.0.0.1
00001101 -> content length = 13
the rest -> name, and then content
```

[source code](https://github.com/CelestialCrafter/celestials-closet/blob/c45cc37ee2e688be766d00bec4ec0621435fc036/src/database.rs)

Thanks for reading! \<3
