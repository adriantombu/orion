Orion
=====

A static site generator written in Rust to create a simple blog from Markdown files.

### CLI Actions
* `orion init [PATH]` : create a new Orion directory to start a new blog (with a few fake posts)
* `orion post [FILENAME]` : create a new Markdown post
* `orion build` : export the Markdown to html
* `orion serve [-b|--build]` : optionnaly build and start a local server to show your blog
* `orion --version` : display the current version of Orion

You can use the `--help` or `-h` flag for each action to know more

### Directory structure
```
.
├── posts
├── config.yaml
├── public
├── static
└── themes
``` 

`/posts`
Contains all the markdown files for your posts

`config.yaml`
The configuration of your site

`/public`
The build directory (the files that will be deployed into production)

`/static`
All the assets you want to use (images for posts, for exemple)

`/themes`
It contains the different themes you can use, each folder representing a them with it's own assets: html layout, css, images, ...

### Front matter
We can use the following values in posts, which will be located at the top of the file in between `---`.

Some of those values can be set globally in the `config.yaml` file

```
---
title: My great title
description: This is a small summary of my post
slug: my-super-slug
canonical:
robots:
published_at:
template:

opengraph:
    type:
    image:
    site_name:

twitter:
    card:
    site:
    creator
---

The actual markdown content
```

### Credits 

The favicon was made by Denis Moskowitz from the [Noun Project](https://thenounproject.com/term/orion/868269/).
