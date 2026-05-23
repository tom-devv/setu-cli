[![Crates.io](https://img.shields.io/crates/v/setu-cli.svg)](https://crates.io/api/v1/crates/setu-cli)
![Crates.io Total Downloads](https://img.shields.io/crates/d/setu-cli)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/tom-devv/setu-cli/test.yml?label=tests)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
# Setu - CLI Markdown Link Checker

`setu` scans your documentation, extracts links, and validates both local file paths and remote URLs concurrently so your docs never suffer from bit rot or broken pathways.

### Motivation
After working on countless projects that all use markdown I find myself running into the same issue, are all my links valid?
A simple typo could redirect users to a dead website or a missing asset and these issues are often caught too late; in production.


## Installation:

Using cargo:

```shell
cargo install setu-cli
```

## Usage:

```
Usage: setu-cli [OPTIONS] [TARGET_PATH]

Arguments:
  [TARGET_PATH]  [default: .]

Options:
  -s, --strict
  -h, --help     Print help
  -V, --version  Print version
```

## Examples:
Use setu to scan the current directory in strict mode
```shell
setu-cli -s
```

To scan a different directory type its path:
```shell
setu-cli ./docs
```

### Concerns Argument

Setu treats remote URL's as OK if _something_ is returned from them, even if thats a `404`, because the site is still _reachable_.
You can flag specific response codes as concerns using the `--concerns` argument: 

```shell
setu-cli --concerns 404,500
```

For example under these rules:

`404 - Faulty`

`401 - Valid`

`500 - Faulty`

## Outputs

Outputs are categorised as `Faulty` or `Invalid`.

### Examples

`FAIL Faulty local link ./does_not_exist.png at ./tests/test.md | The local link does not exist`

`FAIL Faulty remote URL https://simulatehttpcode.vercel.app/statuscode?q=404 at ./tests/test.md | The URL returned an unsuccessful status code: 404` _(note: this is with --concerns 404)_

`FAIL Invalid URL Empty URL at abc.md | Invalid Path`


## CI Examples

To use this as part of your CI see [the workflow used by this repository](https://github.com/tom-devv/Setu/blob/main/.github/workflows/markdown-checker.yml).

When running `setu-cli` as part of your CI, you should run it in `--strict` mode to terminate the program with exit code 1.

## 📄 License

This project is licensed under the [MIT License](https://opensource.org/license/mit)