# Maintainers Notes

This document is intended to capture maintainers notes about this sub project.

## Project structure

1. `compile` contains the code that interacts with Asar.
1. `generated` contains the generated Rust code used by the other projects
1. `src` contains the asm source files. Any file named `mod.asm` will generate a Rust file with its directory name in `generated`
1. `build.rs` is the entry point for the Rust generation.
1. `lib.rs` is the entry point for consuming this project as a library and points to the generated code.

## Notes

1. The file `generate.rs` handles recompilation of game diffs. This is run as part of compilation of the assembly project and
   generates Rust code including the symbol locations and the diffs to apply. To regenerate, remove the hash at the top of the 
   generated rust file and save. If you are running Rust Analyzer, this should automatically happen if there are no Asar 
   compilation errors. You can also run `cargo build` in this subdirectory to explicitly compile Asar if an error occurs.
1. The hash at the top of the generated files is intended to do change detection, but additional work is needed to crawl all
   files to trigger this. The current state is good enough, but a PR is welcome to improve ASM detection if anyone cares enough.
1. Game files cannot be committed and this repository does not use a base diff off of a backed up game file. Instead
   it uses a diff against an empty SNES assumed to be a fastrom file. Any PRs that include game file 
   backups or diffs against game files will be immediately rejected and closed.
1. An obvious limitation of using an empty SNES image is that it is difficult to know where the edges of data have 
   been written. The empty image is filled with `$FF`. Currently, `$FF $FF` is used to detect the boundary of overwritten data since 
   it is unlikely to come up as part of rewriting. The data that uses `$FF $FF` is handled entirely by the Rust code. So it isn't 
   possible to use `db $FF $FF` in this assembly. 
1. Additionally, you shouldn't target two memory locations that are one byte apart. In that case, simply write the middle byte as well. 
   Otherwise, the diff will erroneously include an `$FF` in the middle. This approach has a tradeoff, but it does make it easier to deploy
   in Web Assembly and avoids having a backed up game accidently getting checked in.
