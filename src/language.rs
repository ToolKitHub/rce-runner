use crate::non_empty_vec;
use std::{borrow::Cow, path};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Assembly,
    Ats,
    Bash,
    C,
    Clisp,
    Clojure,
    Cobol,
    CoffeeScript,
    Cpp,
    Crystal,
    Csharp,
    D,
    Dart,
    Elixir,
    Elm,
    Erlang,
    Fsharp,
    Go,
    Groovy,
    Guile,
    Hare,
    Haskell,
    Idris,
    Java,
    JavaScript,
    Julia,
    Kotlin,
    Lua,
    Mercury,
    Nim,
    Nix,
    Ocaml,
    Pascal,
    Perl,
    Php,
    Python,
    Raku,
    Ruby,
    Rust,
    SaC,
    Scala,
    Swift,
    TypeScript,
    Zig,
}

#[derive(Debug)]
pub struct RunInstructions {
    pub build_commands: Vec<Cow<'static, str>>,
    pub run_command: Cow<'static, str>,
}

#[must_use]
pub fn run_instructions(
    language: &Language,
    files: non_empty_vec::NonEmptyVec<path::PathBuf>,
) -> RunInstructions {
    let (main_file, other_files) = files.parts();
    let main_file_str = main_file.to_string_lossy();

    match language {
        Language::Assembly => RunInstructions {
            build_commands: vec![
                format!("nasm -f elf64 -o a.o {main_file_str}").into(),
                "ld -o a.out a.o".into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::Ats => RunInstructions {
            build_commands: vec![
                format!("patscc -o a.out {} {}", main_file_str, source_files(other_files, "dats"))
                    .into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::Bash => RunInstructions {
            build_commands: vec![],
            run_command: format!("bash {main_file_str}").into(),
        },

        Language::C => RunInstructions {
            build_commands: vec![
                format!("clang -o a.out -lm {} {}", main_file_str, source_files(other_files, "c"))
                    .into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::Clisp => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("sbcl --noinform --non-interactive --load {main_file_str}").into(),
        },

        Language::Clojure => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("clj -M {main_file_str}").into(),
        },

        Language::Cobol => RunInstructions {
            build_commands: vec![
                format!("cobc -x -o a.out {} {}", main_file_str, source_files(other_files, "cob"))
                    .into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::CoffeeScript => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("coffee {main_file_str}").into(),
        },

        Language::Cpp => RunInstructions {
            build_commands: vec![
                format!(
                    "clang++ -std=c++11 -o a.out {} {}",
                    main_file_str,
                    source_files(other_files, "c")
                )
                .into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::Crystal => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("crystal run {main_file_str}").into(),
        },

        Language::Csharp => RunInstructions {
            build_commands: vec![
                format!("mcs -out:a.exe {} {}", main_file_str, source_files(other_files, "cs"))
                    .into(),
            ],
            run_command: "mono a.exe".into(),
        },

        Language::D => RunInstructions {
            build_commands: vec![
                format!("dmd -ofa.out {} {}", main_file_str, source_files(other_files, "d")).into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::Dart => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("dart {main_file_str}").into(),
        },

        Language::Elixir => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("elixirc {} {}", main_file_str, source_files(other_files, "ex"))
                .into(),
        },

        Language::Elm => RunInstructions {
            build_commands: vec![format!("elm make --output a.js {main_file_str}").into()],
            run_command: "elm-runner a.js".into(),
        },

        Language::Erlang => RunInstructions {
            build_commands: filter_by_extension(other_files, "erl")
                .iter()
                .map(|file| format!("erlc {}", file.to_string_lossy()).into())
                .collect(),
            run_command: format!("escript {main_file_str}").into(),
        },

        Language::Fsharp => {
            let mut source_files = filter_by_extension(other_files, "fs");
            source_files.reverse();

            RunInstructions {
                build_commands: vec![
                    format!(
                        "fsharpc --out:a.exe {} {}",
                        space_separated_files(&source_files),
                        main_file_str
                    )
                    .into(),
                ],
                run_command: "mono a.exe".into(),
            }
        }

        Language::Go => RunInstructions {
            build_commands: vec![format!("go build -o a.out {main_file_str}").into()],
            run_command: "./a.out".into(),
        },

        Language::Groovy => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("groovy {main_file_str}").into(),
        },

        Language::Guile => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!(
                "guile --no-debug --fresh-auto-compile --no-auto-compile -s {main_file_str}"
            )
            .into(),
        },

        Language::Hare => RunInstructions {
            build_commands: vec![format!("hare build -o a.out {main_file_str}").into()],
            run_command: "./a.out".into(),
        },

        Language::Haskell => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("runghc {main_file_str}").into(),
        },

        Language::Idris => RunInstructions {
            build_commands: vec![format!("idris2 -o a.out --output-dir . {main_file_str}").into()],
            run_command: "./a.out".into(),
        },

        Language::Java => {
            let file_stem = main_file.file_stem().and_then(|s| s.to_str()).unwrap_or("Main");

            RunInstructions {
                build_commands: vec![format!("javac {main_file_str}").into()],
                run_command: format!("java {}", titlecase_ascii(file_stem)).into(),
            }
        }

        Language::JavaScript => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("node {main_file_str}").into(),
        },

        Language::Julia => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("julia {main_file_str}").into(),
        },

        Language::Kotlin => {
            let file_stem = main_file.file_stem().and_then(|s| s.to_str()).unwrap_or("Main");

            RunInstructions {
                build_commands: vec![format!("kotlinc {main_file_str}").into()],
                run_command: format!("kotlin {}Kt", titlecase_ascii(file_stem)).into(),
            }
        }

        Language::Lua => RunInstructions {
            build_commands: vec![],
            run_command: format!("lua {main_file_str}").into(),
        },

        Language::Mercury => RunInstructions {
            build_commands: vec![
                format!("mmc -o a.out {} {}", main_file_str, source_files(other_files, "m")).into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::Nim => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("nim --hints:off --verbosity:0 compile --run {main_file_str}")
                .into(),
        },

        Language::Nix => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("nix-instantiate --eval {main_file_str}").into(),
        },

        Language::Ocaml => {
            let mut source_files = filter_by_extension(other_files, "ml");
            source_files.reverse();

            RunInstructions {
                build_commands: vec![
                    format!(
                        "ocamlc -o a.out {} {}",
                        space_separated_files(&source_files),
                        main_file_str
                    )
                    .into(),
                ],
                run_command: "./a.out".into(),
            }
        }

        Language::Pascal => RunInstructions {
            build_commands: vec![format!("fpc -oa.out {main_file_str}").into()],
            run_command: "./a.out".into(),
        },

        Language::Perl => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("perl {main_file_str}").into(),
        },

        Language::Php => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("php {main_file_str}").into(),
        },

        Language::Python => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("python {main_file_str}").into(),
        },

        Language::Raku => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("raku {main_file_str}").into(),
        },

        Language::Ruby => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("ruby {main_file_str}").into(),
        },

        Language::Rust => RunInstructions {
            build_commands: vec![format!("rustc -o a.out {main_file_str}").into()],
            run_command: "./a.out".into(),
        },

        Language::SaC => RunInstructions {
            build_commands: vec![
                format!(
                    "sac2c -t seq -o a.out {} {}",
                    main_file_str,
                    source_files(other_files, "c")
                )
                .into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::Scala => RunInstructions {
            build_commands: vec![
                format!("scalac {} {}", main_file_str, source_files(other_files, "scala")).into(),
            ],
            run_command: "scala Main".into(),
        },

        Language::Swift => RunInstructions {
            build_commands: vec![
                format!("swiftc -o a.out {} {}", main_file_str, source_files(other_files, "swift"))
                    .into(),
            ],
            run_command: "./a.out".into(),
        },

        Language::TypeScript => RunInstructions {
            build_commands: vec![
                format!("tsc -outFile a.js {} {}", main_file_str, source_files(other_files, "ts"))
                    .into(),
            ],
            run_command: "node a.js".into(),
        },

        Language::Zig => RunInstructions {
            build_commands: Vec::new(),
            run_command: format!("zig run {main_file_str}").into(),
        },
    }
}

fn source_files(files: Vec<path::PathBuf>, extension: &str) -> String {
    space_separated_files(&filter_by_extension(files, extension))
}

fn filter_by_extension(files: Vec<path::PathBuf>, extension: &str) -> Vec<path::PathBuf> {
    files
        .into_iter()
        .filter(|file| file.extension().and_then(|s| s.to_str()) == Some(extension))
        .collect()
}

fn space_separated_files(files: &[path::PathBuf]) -> String {
    files.iter().map(|file| file.to_string_lossy()).collect::<Vec<Cow<str>>>().join(" ")
}

fn titlecase_ascii(s: &str) -> Cow<'_, str> {
    if !s.is_ascii() || s.len() < 2 {
        s.into()
    } else {
        let (head, tail) = s.split_at(1);
        format!("{}{}", head.to_ascii_uppercase(), tail).into()
    }
}
