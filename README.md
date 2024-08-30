This is an improved version with various bugfixes (including a crash/corruption bug due to misaligned blocks). It also works on FreeBSD and NetBSD (if it does not work on NetBSD, please patch the fuse-rs crate with the files from `fuse/`). This work builds up on the amazing work by Naruaki Matsumura (who I know as [@narumatt](https://github.com/narumatt)).

Code and fixes here (`sqlitefs` repo) are dual licensed under MIT/Apache-2.0

Please star the repository if you find it interesting or useful. For a monotemporal filesystem built on the same (which records everything so you can do point-in-time inspections), please see [temporal_sqlitefs](https://github.com/ris-work/sqlitefs_temporal). It is available under OSLv3, which is approved by both the OSI and the FSF, but is more restrictive than dual-licensing under Apache-2.0 and MIT. 

# sqlite-fs

## About

sqlite-fs allows Linux and MacOS to mount a sqlite database file as a normal filesystem.

## Requirements

- Latest Rust Programming Language (≥ 1.38)
- libfuse(Linux) or osxfuse(MacOS) is requied by [fuse-rs](https://github.com/zargony/fuse-rs)

## Usage
### Mount a filesystem

```
$ sqlite-fs <mount_point> [<db_path>]
```

If a database file doesn't exist, sqlite-fs create db file and tables.

If a database file name isn't specified, sqlite-fs use in-memory-db instead of a file.
All data will be deleted when the filesystem is closed.

### Unmount a filesystem

- Linux

```
$ fusermount -u <mount_point>
```

- Mac

```
$ umount <mount_point>
```

## example
```
$ sqlite-fs ~/mount ~/filesystem.sqlite &
$ echo "Hello world\!" > ~/mount/hello.txt
$ cat ~/mount/hello.txt
Hello world!
```

## functions

- [x] Create/Read/Delete directories
- [x] Create/Read/Write/Delete files
- [x] Change attributions
- [x] Copy/Move files
- [x] Create Hard Link and Symbolic Link
- [x] Read/Write extended attributes
- [] File lock operations
- [] Strict error handling

