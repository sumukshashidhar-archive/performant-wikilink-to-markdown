# performant-wikilink-to-markdown
High Performant Conversion of Referenced Wikilinks to Markdown. Use with LogSeq, Obsidian, etc. The purpose of this is to allow you to use a reference style for wikilinks in your markdown files, and then convert them to the markdown format for posterity and backups.


## Usage

```bash
cargo run  <input_directory> <output_directory>
```

## Example

If you have a reference like:

```
This is a [[wikilink]].
```

and have a file in a subdirectory of the input directory like:

```
./subdir/x/wikilink.md
```

then the program will replace the wikilink with the reference:

```
This is a [wikilink](./subdir/x/wikilink.md).
```

# Contributing

I'm happy to accept pull requests for any enhancements. One potential idea that I have is implementing parallel processing, but that may be slightly more complicated than I want to get into at the moment. I'm also happy to accept issues for bugs or feature requests.