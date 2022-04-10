use std::env;
use std::process;

const PROGRAM_NAME: &str = "rbasename";
const EXIT_SUCCESS: i32 = 0;
const EXIT_ERROR: i32 = 1;

fn usage() {
    println!("Usage: {} NAME [SUFFIX]
Print NAME removing all directories.
If specified, also remove a trailling SUFFIX.

Examples:
  rbasename /users/user/my_file.txt       -> \"my_file.txt\"
  rbasename /users/user/my_file.txt .txt  -> \"my_file\"
  rbasename include/stdio.h .h            -> \"stdio\"

This program was inspired on GNU coreutils basename.
        ", PROGRAM_NAME);
}

fn main() {
    for arg in env::args() {
        if arg == "--help" {
            usage();
            process::exit(EXIT_SUCCESS);
        }
    }

    let args: Vec<String> = env::args().collect();

    let f_fname;
    let mut suffix = "";
    match args.len() {
        1 => {
            eprintln!("{0}: missing NAME. {0} --help to see usage", PROGRAM_NAME);
            process::exit(EXIT_ERROR);
        },
        2 => f_fname = &args[1],
        3 => {
            f_fname = &args[1];
            suffix = &args[2];
        },
        _ => {
            eprintln!("Too many arguments. {} --help to see usage", PROGRAM_NAME);
            process::exit(EXIT_ERROR);
        }
    }

    let fname = get_fname(f_fname, suffix);

    println!("{0}", fname);
    process::exit(EXIT_SUCCESS);
}

fn get_fname(f_fname: &String, suffix: &str) -> String {
    let dirs: Vec<&str> = f_fname.split("/").collect();
    
    let mut fname: &str;
    match dirs.len() {
        1 => fname = &dirs[0],
        _ => {
            let di = dirs.len()-1;
            fname = if !dirs[di].is_empty() {
                &dirs[di]
            } else {
                &dirs[di-1]
            };
        },
    }

    if fname.is_empty() {
        fname = "/";
    }

    if fname.ends_with(suffix) && fname.len() > suffix.len() {
        fname = &fname[..(fname.len()-suffix.len())];
    }

    fname.to_string()
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_without_suffix() {
        assert_eq!(get_fname(&"/users/myuser/projects/amazing_rust".to_string(), ""), "amazing_rust".to_string());
    }

    #[test]
    fn test_with_slash_at_end() {
        assert_eq!(get_fname(&"/users/myuser/".to_string(), ""), "myuser".to_string());
    }

    #[test]
    fn test_with_single_slash() {
        assert_eq!(get_fname(&"/".to_string(), ""), "/".to_string());
    }

    #[test]
    fn test_with_suffix() {
        assert_eq!(get_fname(&"/user/myuser/file.rs".to_string(), ".rs"), "file".to_string());
    }

    #[test]
    fn test_with_diff_suffix() {
        assert_eq!(get_fname(&"include/stdio.h".to_string(), ".rs"), "stdio.h");
    }

    #[test]
    fn test_with_long_suffix() {
        assert_eq!(get_fname(&"include/stdio.h".to_string(), "stdio.h"), "stdio.h");
    }
}
