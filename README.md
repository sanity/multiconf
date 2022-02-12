# MultiConf

A command line tool for easily generating multiple versions of a configuration file
from a single template.

## Why?

I'm a big fan of the [i3 window manager](https://i3wm.org/), and I use it on several
linux desktops and laptops. While I mostly want the same i3 configuration across
all of these computers, some require different variations.

I created multiconf to make it easy to generate multiple variations of a config file.

## How to use

Create a file called `base.txt`:

```
global config 1
global config 2
dog$>>canine config 1
cat$>>feline config 1
global config 3
```

Then create the "dog" version of `base.txt`:

```
$ multiconf -c dog -i base.txt -o dog.txt
```

This will contain:

```
global config 1
global config 2
canine config 1
global config 3
```

## Command line parameters
```
multiconf 0.1.3
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