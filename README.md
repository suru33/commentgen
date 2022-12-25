# commentgen

Generate consistent length comment

```shell
cargo install --git https://github.com/suru33/commentgen.git
```

```
$ commentgen -h
usage: commentgen [options] [COMMENT]

Options:
    -l, --language      Programing language (default: shell)
    -L, --length        Comment length (default: 80)
    -h, --help          Print help menu
```

## Example:

```
$ commentgen -l html 'some comment'
<!- ----------------------------- some comment ------------------------------ ->
```

## Supported languages

- shell
- java
- scala
- kotlin
- rust
- r
- objective-c
- erlang
- perl
- ruby
- elixir
- swift
- go
- curl
- julia
- haskell
- clojure
- python
- js
- ts
- sql
- c
- cpp
- csharp
- html
- xml
- php
- lua
- ps
- lisp
- pascal
- jsp
