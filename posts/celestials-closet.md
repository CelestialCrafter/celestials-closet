---
title = "building my website"
summary = "aaaaaaaaaaaaa"
date = 2025-04-10
id = "celestials-closet"
---

## @TODO 

table of content based on h2's
publish date + revision date
display lang/origin in codeblocks
404 page
gallery
custom templating engine

## motivation

## comptime post parsing

to make the site static, the posts needed to be built in too.
i didnt want to use code on the client, because i want the frontend to be as light as possible.
i decided to use [pulldown-cmark](https://crates.io/crates/pulldown-cmark) as my markdown parser because:
	- its fast (obviously)
	- its iterator-style implementation allows for really cool things (heading links, syntax highlighting)

### syntax highlighting w/ treesitter

i didnt use something like [syntect](https://crates.io/crates/syntect) because:
	- im not (and dont care to be) familiar with sublime text syntax definitions, because i dont use sublime text!! i use neovim, which uses treesitter under the hood
	- treesitter is a cool project to me, and i felt like using it in something

anyways, lets go over the syntax highlighting starting from the parser iterator:

```rust
parser
	.scan(None, move |state, event| {
	    let inside = state.is_some();
	    match event {
	        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref language))) => {
	            *state = get_config(language)
	        }
	        Event::Text(ref code) if inside => {
	            match highlight_code(highlighter, state.unwrap(), code) {
	                Ok(highlighted) => return Some(Some(Event::Html(CowStr::from(highlighted)))),
	                Err(err) => println!("cargo:warning=could not highlight {}: {}", code, err),
	            }
	        }
	        Event::End(TagEnd::CodeBlock) if inside => *state = None,
	        _ => (),
	    }
	
	    Some(Some(event))
	})
	.filter_map(|e| e)
```

this code:
- finds codeblocks & stores their language
- finds their text blocks, and injects the syntax highlighted html from the next function

now its time for the actual highlighting part! its really easy to follow compared to the rest:

```rust
let highlights = highlighter.highlight(config, code.as_bytes(), None, |_| None)?;

let mut html = String::with_capacity(code.len());

for event in highlights.map(|e| {
    e.unwrap_or_else(|err| {
        println!("cargo:warning=highlight error: {}", err);
        HighlightEvent::HighlightEnd
    })
}) {
    match event {
        HighlightEvent::Source { start, end } => escape_html(&mut html, &code[start..end]),
        HighlightEvent::HighlightStart(highlight) => {
            write!(html, "<span class=\"{}\"/>", HIGHLIGHT_NAMES[highlight.0])
        }
        HighlightEvent::HighlightEnd => write!(html, "</span>"),
    }
    .unwrap();
}

Ok(html)
```

this simply gets the highlights, iterates over them, and spits out the colored source code according to the highlights

here's the [source code](https://github.com/CelestialCrafter/celestials-closet/blob/62d1c105a2cb8d6536ee2fda3f5b7a933edeba5d/build/highlighting.rs) for the rest of the stuff i didnt cover.

### heading links

this one was easier, using [Iterator::scan()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan) again:

```rust
parser
	.scan(None, |state, event| {
	    match event {
	        Event::Text(ref text) => {
	            if let Some(Tag::Heading {
	                ref mut id, level, ..
	            }) = state
	            {
	                let new_id = text.to_lowercase().replace(" ", "-");
	                let link = Tag::Link {
	                    link_type: LinkType::Inline,
	                    dest_url: format!("#{}", new_id).into(),
	                    title: CowStr::Borrowed(""),
	                    id: CowStr::Borrowed(""),
	                };
	                *id = Some(new_id.into());
	                let level = level.clone();

	                return Some(vec![
	                    Event::Start(state.take().unwrap()),
	                    Event::Start(link),
	                    event,
	                    Event::End(TagEnd::Link),
	                    Event::End(TagEnd::Heading(level)),
	                ]);
	            }
	        }
	        Event::Start(tag @ Tag::Heading { id: None, .. }) => {
	            *state = Some(tag);
	            return Some(vec![]);
	        }
	        Event::End(TagEnd::Heading(_)) => return Some(vec![]),
	        _ => (),
	    }

	    Some(vec![event])
	})
	.flatten()
```

this just highjacks the headings, and wraps them in anchor links

## embedding to static binary

## custom data format

## star background
