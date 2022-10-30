use std::fs;
use thiserror::Error;

/// Create a new generic post
pub fn run(file_slug: &str) -> Result<(), NewPostError> {
    let now = chrono::offset::Utc::now().format("%Y-%m-%d");
    let path = format!("posts/{}-{}.md", now, slug::slugify(file_slug));

    println!("Creating a new post to {}", path);

    let template = "---
title: I'm an amazing title
description: I'm a short description of my amazing post
published_at: 2022-11-01 13:42:37
---

# Lorem ipsum dolor sit amet,

Suspendisse pellentesque, **urna ut porttitor faucibus**, elit nulla lacinia mauris, at pulvinar lacus nisl eu orci. Nullam dui libero, commodo in sollicitudin nec, rutrum ac nisl.

Phasellus eleifend at nunc a molestie :

- Quisque ullamcorper felis et *urna scelerisque aliquet*.
- Nam ipsum lacus, volutpat in tempor et, volutpat in tortor.
- Ut tempus sem eu rhoncus placerat.
- Donec eleifend fermentum odio, a aliquam urna varius posuere.

Vestibulum aliquet metus nulla, sit [amet ultrices dolor](index.html) sodales ac. Morbi risus quam, sagittis et augue eu, rhoncus imperdiet odio. Aenean ac condimentum ipsum. 
";

    Ok(fs::write(path, template)?)
}

#[derive(Error, Debug)]
pub enum NewPostError {
    #[error("{0}")]
    StdIo(#[from] std::io::Error),
}
