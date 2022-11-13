# findr

### Rust implementation of the unix 'find' tool

```
USAGE:
    findr [OPTIONS] [--] [PATH]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <NAME>...    Name
    -t, --type <TYPE>...    Entry type [possible values: d, f, l]

ARGS:
    <PATH>...    Search paths [default: .]
```