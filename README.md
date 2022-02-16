# MultiConf

A command line tool for easily generating multiple versions of a configuration file
from a single template.

## Why?

I'm a big fan of the [i3 window manager](https://i3wm.org/), and I use it on several
linux desktops and laptops. While I mostly want the same i3 configuration across
all of these computers, some require different variations.

I created multiconf to make it easy to generate multiple variations of a config file.

## How to install

1. Install cargo (the Rust package manager):
```bash
$ curl https://sh.rustup.rs -sSf | sh
```
2. Install multiconf:
```bash
$ cargo install multiconf
```

## How to use

Create a file called `base.txt`:

```
global config 1
global config 2
dog$>>canine config 1
cat$>>feline config 1
global config 3
```

Then create `dog.txt` from `base.txt`:

```bash
$ multiconf --choice dog --input base.txt --output dog.txt
```

This will contain:

```
global config 1
global config 2
canine config 1
global config 3
```

If you want dog.txt to be automatically updated every time base.txt changes, use:

```bash
$ multiconf --choice dog --input base.txt --output dog.txt --watch
```

## Example with i3 config file

Here is the line I use in my i3 config so I can have different versions depending on the hostname of the computer:

```bash
exec --no-startup-id multiconf \
    --choice `hostname` \
    --input ~/Dropbox/code/linux-cfg/i3/config \
    --output ~/.config/i3/config \
    --watch
```

Here are some example lines from my i3 config, which I share between machines
using Dropbox:

```
# This font is widely installed, provides lots of unicode glyphs, right-to-left
# text rendering and scalability on retina/hidpi displays (thanks to pango).
technic$>>font pango:DejaVu Sans Mono 12
pocket$>>font pango:DejaVu Sans Mono 16
framework$>>font pango:DejaVu Sans Mono 15
```

`technic`, `pocket`, and `framework` are the hostnames of my three linux computers.

## Command line parameters
```
multiconf [version]
Easily generate different versions of config files

USAGE:
    multiconf [OPTIONS] --choice <CHOICE>

OPTIONS:
    -c, --choice <CHOICE>          The selected
    -h, --help                     Print help information
    -i, --input <INPUT>            The input file, or stdin if not specified
    -o, --output <OUTPUT>          The output file, or stdout if not specified
    -s, --separator <SEPARATOR>    Separates choice from the line chosen [default: $>>]
    -V, --version                  Print version information
    -w, --watch                    Watch the input file for changes and update the output file when
                                   it does
```
