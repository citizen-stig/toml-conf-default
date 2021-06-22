# Default values for Toml config

It works using serde:

```
$ cargo run --color=always --package toml-conf-default --bin toml-conf-default -- sample_config.toml

Parsed toml: Ok(Config { some_number: 42, some_parameter: "default top level", extra_section: ExtraSection { another_number: 33, another_parameter: "default_extra" } })
===========
some_number = 111
some_parameter = "default top level"

[extra_section]
another_number = 33
another_parameter = "default_extra"
```
