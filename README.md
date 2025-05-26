# rce-runner

## What is this?

**rce-runner** is a command line tool that runs your code snippets in many different programming languages. You give it code, it runs the code, and gives you back the results.

## When would I use this?

- You're building a website where users can write and run code
- You need to test code in many languages without installing all those languages
- You're creating a learning platform and need to execute student code
- You're building tools for technical interviews or coding challenges

## Installation

Run this command to install rce-runner:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ToolKitHub/rce-runner/releases/download/v1.2.4/rce-runner-installer.sh | sh
```

## Quick Start Guide

### What you'll need

- A system with the programming languages you want to run installed
- Basic familiarity with JSON format

### Basic Example

Send this JSON as input:

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

You'll get this output:

```json
{
  "stdout": "Hello, world!\n",
  "stderr": "",
  "error": ""
}
```

## Supported Languages

rce-runner supports 40+ programming languages:

| Language | File Extension |
|----------|---------------|
| Assembly | .asm |
| Ats | .dats |
| Bash | .sh |
| C | .c |
| Clisp | .lisp |
| Clojure | .clj |
| Cobol | .cob |
| CoffeeScript | .coffee |
| Cpp (C++) | .cpp |
| Crystal | .cr |
| Csharp (C#) | .cs |
| D | .d |
| Dart | .dart |
| Elixir | .ex |
| Elm | .elm |
| Erlang | .erl |
| Fsharp (F#) | .fs |
| Go | .go |
| Groovy | .groovy |
| Guile | .scm |
| Hare | .ha |
| Haskell | .hs |
| Idris | .idr |
| Java | .java |
| JavaScript | .js |
| Julia | .jl |
| Kotlin | .kt |
| Lua | .lua |
| Mercury | .m |
| Nim | .nim |
| Nix | .nix |
| Ocaml | .ml |
| Pascal | .pas |
| Perl | .pl |
| Php | .php |
| Python | .py |
| Raku | .raku |
| Ruby | .rb |
| Rust | .rs |
| SaC | .c |
| Scala | .scala |
| Swift | .swift |
| TypeScript | .ts |
| Zig | .zig |


> [!TIP] 
>
> Can't find your preferred programming language? [Open an issue](https://github.com/ToolKitHub/rce-runner/issues/new) or send a PR to request support.

## License

See [License](LICENSE)
