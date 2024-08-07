Orion
=====

A static site generator written in Rust to create a simple blog from Markdown files.

### CLI Actions

* `orion init --path my-blog` : create a directory to start a new Orion blog (with a few fake posts)
* `orion post --slug my-amazing-title --draft` : create a new Markdown post
* `orion build` : export the Markdown to html
* `orion serve` : build, start a local server to test your build, and watch for changes on `/posts`, `/static`
  and `/themes`
* `orion --version` : display the current version of Orion
* `orion --help` : print the help

### Directory structure

```
.
├── posts
├── build.sh
├── config.toml
├── public
├── static
└── themes
``` 

`/posts`
Contains all the markdown files for your posts

`build.sh`
A build script to use in CI/CD environments

`config.toml`
The configuration of your site

`/public`
The build directory (the files that will be deployed into production)

`/static`
All the assets you want to use (images for your posts, for example)

`/themes`
It contains the different themes you can use, each folder representing them with its own assets: html layout, css,
images, ...

### Front matter

You can use the following values in your posts. They will be located at the top of the file in between `---`.

```
---
title: My great title
description: This is a small summary of my post
published_at: 2022-11-01 11:12:13
image: https://www.publicdomainpictures.net/pictures/220000/velka/orion-nebula.jpg
---

The actual markdown content
```

### Credits

The favicon was made by Denis Moskowitz from the [Noun Project](https://thenounproject.com/term/orion/868269/).
