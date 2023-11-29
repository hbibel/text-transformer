# CLI Argument Order

## Problem

We want to be able to pass the program source as a CLI argument. There's
different approaches to achieve that, e.g.:

```sh
# 1. follow awk, program source must be passed as last argument
tt -foo bar -baz [ -- ] <program source> file ...
# 2. program source must be passed either before or after a --
tt -foo bar -baz <program source> -- <input file>
tt -foo bar -baz -- <program source> <input file>
```

Option 2 has the advantage that the program can be passed without quotes, e.g.,

```sh
tt -v do this and that -- input_file.txt
```

However it makes it hard to identify mistyped arguments, e.g.

```sh
# Oops, forgot the "-"
tt v do this and that -- input_file.txt
```

In this example, the `v` would be interpreted as part of the program source,
which means that our program would fail later, when the program is interpreted.

## Decision

`awk`s strikes a good balance between usability and identifying misusing, so
we'll implement the same option syntax.
