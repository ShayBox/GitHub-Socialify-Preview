# GitHub Socialify Preview

Upload GitHub repository Socialify preview images

Based on [mheap/github-social-image](https://github.com/mheap/github-social-image)

## Command Line

`$ github-socialify-preview --help`
```
Upload GitHub repository Socialify preview images

Usage: github-socialify-preview.exe [OPTIONS] --repo <REPO>

Options:
  -r, --repo <REPO>                GitHub Repository (Example: ShayBox/GitHub-Social-Preview)
  -t, --theme <THEME>              Socialify Theme [possible values: light, dark, auto]
  -f, --font <FONT>                Socialify Font [possible values: inter, bitter, raleway, rokkitt, source-code-pro, ko-ho, jost]
  -p, --pattern <PATTERN>          Socialify Background Pattern [possible values: signal, charlie-brown, formal-invitation, plus, circuit-board, overlapping-hexagons, brick-wall, floating-cogs, diagonal-stripes, solid]
  -l, --logo <LOGO>                Socialify SVG Logo (Image URL or Data URI)
      --owner                      Socialify Show Owner Name
      --language                   Socialify Show Language
      --stargazers                 Socialify Show Stars Count
      --forks                      Socialify Show Forks Count
      --issues                     Socialify Show Issues Count
      --pulls                      Socialify Show Pull Requests Count
  -d, --description <DESCRIPTION>  Socialify Show Description
  -h, --help                       Print help
  -V, --version                    Print version
```

## Installation

[Cargo]/[Crate]: `$ cargo install github-socialify-preview`  
[GitHub Latest Release (Portable)](https://github.com/ShayBox/GitHub-Socialify-Preview/releases/latest)

[Cargo]: https://rustup.rs
[Crate]: https://crates.io/crates/github-socialify-preview