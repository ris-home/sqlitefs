#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
use clap::{App, Arg};
use sqlite_temporal_fs::db_module::sqlite::Sqlite;
use sqlite_temporal_fs::db_module::DbModule;
use sqlite_temporal_fs::filesystem::SqliteFs;
use std::env;
use std::ffi::OsStr;
#[allow(unused_parens)]

fn main() {
    env_logger::init();

    let mount_option_arg = Arg::with_name("mount_option")
        .short('o')
        .long("option")
        .help("Additional mount option for this filesystem")
        .takes_value(true)
        .multiple(true);

    let mount_point_arg = Arg::with_name("mount_point")
        .help("Target mountpoint path")
        .index(1)
        .required(true);

    let db_path_arg = Arg::with_name("db_path")
        .help("Sqlite database file path. If not set, open database in memory. Rewind is not supported if an in-memory database is used.")
        .index(2);

    let db_syn_mode_arg = Arg::with_name("syn_mode")
        .short('s')
        .long("syn_mode")
        .help("Sqlite database sync mode.")
        .takes_value(true);

    let db_no_time_recording_arg = Arg::with_name("no_time_recording")
        .short('n')
        .long("no_time")
        .help("Don't record atime, ctime, etc. It will make it significantly faster for on-disk databases.");

    let db_read_only_arg = Arg::with_name("read_only")
        .short('r')
        .long("read-only")
        .help("Mount as read-only.");

    let db_rollback_mode_arg = Arg::with_name("rollback_mode")
        .short('b')
        .long("rollback-mode")
        .help("Rollback instead of WAL mode.");

    let license_arg = Arg::with_name("display_license")
        .short('L')
        .long("license")
        .help("Display the license.");

    let matches = App::new("temporal-sqlite-fs")
        .about("Sqlite database as a filesystem.")
        .version(crate_version!())
        .arg(mount_option_arg)
        .arg(mount_point_arg)
        .arg(db_path_arg)
        .arg(db_no_time_recording_arg)
        .arg(db_read_only_arg)
        .arg(db_rollback_mode_arg)
        .arg(db_syn_mode_arg)
        .arg(license_arg)
        .get_matches();

    let mut option_vals = [
        "-o",
        "fsname=temporal-sqlite-fs",
        "-o",
        "default_permissions",
        "-o",
        "allow_other",
        "-o",
        "max_write=16384",
        "-o",
        "max_read=16384",
    ]
    .to_vec();
    if let Some(v) = matches.values_of("mount_option") {
        for i in v {
            option_vals.push("-o");
            option_vals.push(i);
        }
    }

    const LICENSE: &str = include_str!("../LICENSE.txt");

    let mountpoint = matches
        .value_of("mount_point")
        .expect("Mount point path is missing.");
    let db_path = matches.value_of("db_path");
    let db_syn_mode_requested = matches.value_of("syn_mode");
    let db_syn_mode;

    match db_syn_mode_requested {
        Some(s) => db_syn_mode = s,
        None => db_syn_mode = "FULL",
    }
    let db_read_only: bool = matches.is_present("read_only");
    let db_no_time: bool = matches.is_present("no_time_recording");
    let display_license: bool = matches.is_present("display_license");
    let db_rollback_mode: bool = matches.is_present("rollback_mode");
    if (display_license) {
        println!("{}", LICENSE)
    }
    let options = option_vals
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();
    let fs: SqliteFs;
    debug!("read-only: {}", db_read_only);
    match db_path {
        Some(path) => {
            debug!("Rewind is not requested; proceeding as-is.");
            if (db_read_only) {
                fs = match SqliteFs::new_read_only(path) {
                    Ok(n) => n,
                    Err(err) => {
                        println!("{:?}", err);
                        return;
                    }
                };
            } else {
                if (db_no_time) {
                    fs = match SqliteFs::new_no_time_recording(path, !db_rollback_mode, db_syn_mode)
                    {
                        Ok(n) => n,
                        Err(err) => {
                            println!("{:?}", err);
                            return;
                        }
                    };
                } else {
                    fs = match SqliteFs::new(path, !db_rollback_mode, db_syn_mode) {
                        Ok(n) => n,
                        Err(err) => {
                            println!("{:?}", err);
                            return;
                        }
                    };
                }
            }
        }
        None => {
            let mut db = match Sqlite::new_in_memory() {
                Ok(n) => n,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            };
            match db.init() {
                Ok(n) => n,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            };
            fs = match SqliteFs::new_with_db(db) {
                Ok(n) => n,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            };
        }
    }
    match fuse::mount(fs, &mountpoint, &options) {
        Ok(n) => n,
        Err(err) => error!("{}", err),
    }
}
