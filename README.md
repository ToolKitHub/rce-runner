# rce-runner

A command line application that reads code as a json payload from stdin, compiles and runs the code, and writes
the result as json to stdout.

Used by [rce-images](https://github.com/ToolKitHub/rce-images) to run code.

## Payload Format

Input:

```json
{
  "language": "python",
  "files": [
    {
      "name": "main.py",
      "content": "print('Hello, world!')"
    }
  ]
}
```

Output:

```json
{
  "stdout": "Hello, world!\n",
  "stderr": "",
  "error": ""
}
```

## Supported Programming Languages

| Language     | File Extension |
| ------------ | -------------- |
| Assembly     | .asm           |
| Ats          | .dats          |
| Bash         | .sh            |
| C            | .c             |
| Clisp        | .lisp          |
| Clojure      | .clj           |
| Cobol        | .cob           |
| CoffeeScript | .coffee        |
| Cpp (C++)    | .cpp           |
| Crystal      | .cr            |
| Csharp (C#)  | .cs            |
| D            | .d             |
| Dart         | .dart          |
| Elixir       | .ex            |
| Elm          | .elm           |
| Erlang       | .erl           |
| Fsharp (F#)  | .fs            |
| Go           | .go            |
| Groovy       | .groovy        |
| Guile        | .scm           |
| Hare         | .ha            |
| Haskell      | .hs            |
| Idris        | .idr           |
| Java         | .java          |
| JavaScript   | .js            |
| Julia        | .jl            |
| Kotlin       | .kt            |
| Lua          | .lua           |
| Mercury      | .m             |
| Nim          | .nim           |
| Nix          | .nix           |
| Ocaml        | .ml            |
| Pascal       | .pas           |
| Perl         | .pl            |
| Php          | .php           |
| Python       | .py            |
| Raku         | .raku          |
| Ruby         | .rb            |
| Rust         | .rs            |
| SaC          | .c             |
| Scala        | .scala         |
| Swift        | .swift         |
| TypeScript   | .ts            |
| Zig          | .zig           |

> [!TIP]
>
> Can't find your preferred programming language? [Open an issue](https://github.com/ToolKitHub/rce-runner/issues/new) to request support.

## License

This project is licensed under the [MIT License](LICENSE).
