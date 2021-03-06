Orion  [![Buy me a tree](https://img.shields.io/badge/Buy%20me%20a%20tree-%F0%9F%8C%B3-lightgreen)](https://offset.earth/adrian)
=====

A static site generator written in Go to create a simple blog from Markdown files.

This is not intended to create complex websites, the excellent [Hugo](https://gohugo.io/) does that extremely well already!

### CLI Actions
* `orion version` : displays the current version of Orion
* `orion init` : creates a new Orion directory to start a new blog (with 2 or 3 fake articles)
* `orion article article-file-name` : creates a new markdown article (may be a draft, hence not published)
* `orion build` : exports the markdown to html
* `orion serve` : builds and starts a small server to show your blog

You can use the `--help` or `-h` flag for each action to know more

### Directory structure
```
.
├── articles
├── config.yaml
├── public
├── static
└── themes
``` 

`/articles`
Contains all the markdown files for your articles

`config.yaml`
The configuration of your site

`/public`
The build directory (the files that will be deployed into production)

`/static`
All the assets you want to use (images for articles, for exemple)

`/themes`
It contains the different themes you can use, each folder representing a them with it's own assets: html layout, css, images, ...

### Front matter
We can use the following values in articles, which will be located at the top of the file in between `---`.

Some of those values can be set globally in the `config.yaml` file

```
---
title: My great title
description: This is a small summary of my article
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

### How to build and package the assets

```
chmod +X ./scripts/build.sh
./scripts/build.sh
```

### Credits 

The favicon was made by Denis Moskowitz from the [Noun Project](https://thenounproject.com/term/orion/868269/).
