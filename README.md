# NF Scout

A Rust CLI tool that scans a folder and generates a categorized Markdown file report.

## What it does

Give it a folder path. It recursively finds every .md file, groups them by
top-level subfolder, flags empty or near-empty files, and writes a report.

Built for Markdown-heavy vaults and creative project folders.

## Usage

`cargo run -- "path/to/your/folder"`
 
 
**Optional:** specify a custom output path for the report

`cargo run -- "path/to/your/folder" "path/to/output/report.md"`


If no output path is given, the report saves to the current directory as
`{folder-name}_report.md`.

## Example output


NF Scout Report

Generated: 2026-07-11 14:32

Project path:
D:/TTRPG/porject_folder_path

Markdown files found: 77

Categories detected:
- Bestiary: 12
- Geography: 28
- Artifacts: 4

Possible issues:
Empty or stub files (6 found):
- D:/TTRPG/.../placeholder.md


### Requirements

- Rust

#### Build

`cargo build --release`


### Status

v0.1 — core scanning and reporting functional.
