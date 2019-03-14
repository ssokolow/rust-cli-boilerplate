/*! Validator functions suitable for use with `Clap` and `StructOpt`

Copyright 2017-2019, Stephan Sokolow
*/

use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

/// Special filenames which cannot be used for real files under Win32
///
/// (Unless your app uses the `\?\` path prefix to bypass legacy Win32 API compatibility
/// limitations)
///
/// Source: [Boost Path Name Portability Guide
/// ](https://www.boost.org/doc/libs/1_36_0/libs/filesystem/doc/portability_guide.htm)
const RESERVED_DOS_FILENAMES: &[&str] = &["AUX", "CON", "NUL", "PRN",
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    "CLOCK$" ]; // https://www.boost.org/doc/libs/1_36_0/libs/filesystem/doc/portability_guide.htm
// TODO: Add the rest of the disallowed names from
// https://en.wikipedia.org/wiki/Filename#Comparison_of_filename_limitations

/// The given path can be opened for reading
///
/// ## Use For:
///  * Input file paths
///
/// ## Relevant Conventions:
///  * Commands which read from `stdin` by default should use `-f` to specify the
///    input path. [[1]](http://www.catb.org/esr/writings/taoup/html/ch10s05.html)
///  * Commands which read from files by default should use positional arguments to
///    specify input paths.
///  * Allow an arbitrary number of input paths if feasible.
///  * Interpret a value of `-` to mean "read from `stdin`" if feasible.
///    [[2]](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html)
///
/// **Note:** The following command-lines, which interleave files and `stdin`, are a good test of
/// how the above conventions should interact:
///
///     data_source | my_utility_a header.dat - footer.dat > output.dat
///     data_source | my_utility_b -f header.dat -f - -f footer.dat > output.dat
///
/// ## Cautions:
///  * This will momentarily open the given path for reading to verify that it is
///    readable. However, relying on this to remain true will introduce a race condition. This
///    validator is intended only to allow your program to exit as quickly as possible in the case
///    of obviously bad input.
///  * As a more reliable validity check, you are advised to open a handle to the file in question
///    as early in your program's operation as possible and keep it open until you are finished.
///    This will both verify its validity and minimize the window in which another process could
///    render the path invalid.
///
/// **TODO:** Determine why `File::open` has no problem opening directory paths and decide how to
/// adjust this.
///
pub fn path_readable<P: AsRef<Path> + ?Sized>(value: &P) -> std::result::Result<(), OsString> {
    let path = value.as_ref();
    File::open(path)
        .map(|_| ())
        .map_err(|e| format!("{}: {}", path.display(), e).into())
}

/// The given path is valid on all major filesystems and OSes
///
/// ## Use For:
///  * Output file or directory paths
///
/// ## Relevant Conventions:
///  * Use `-o` to specify the output path.
///    [[1]](http://www.catb.org/esr/writings/taoup/html/ch10s05.html)
///    [[2]](http://tldp.org/LDP/abs/html/standard-options.html)
///  * Interpret a value of `-` to mean "Write output to stdout".
///    [[3]](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html)
///  * Because `-o` does not inherently indicate whether it expects a file or a directory, consider
///    also providing a GNU-style long version with a name like `--outfile` to allow scripts which
///    depend on your tool to be more self-documenting.
///
/// ## Cautions:
///  * To ensure files can be copied/moved without issue, this validator may impose stricter
///    restrictions on filenames than your filesystem. Do *not* use it for input paths.
///  * Other considerations, such as paths containing symbolic links with longer target names, may
///    still cause your system to reject paths which pass this check.
///  * As a more reliable validity check, you are advised to open a handle to the file in question
///    as early in your program's operation as possible and keep it open until you are finished.
///    This will both verify its validity and minimize the window in which another process could
///    render the path invalid.
///
/// ## Design Considerations: [[4]](https://en.wikipedia.org/wiki/Comparison_of_file_systems#Limits)
///  * Many popular Linux filesystems impose no total length limit.
///  * This function imposes a 32,760-character limit for compatibility with flash drives formatted
///    FAT32 or exFAT.
///  * Some POSIX API functions, such as `getcwd()` and `realpath()` rely on the `PATH_MAX`
///    constant, which typically specifies a length of 4096 bytes including terminal `NUL`, but
///    this is not enforced by the filesystem itself.
///    [[4]](https://insanecoding.blogspot.com/2007/11/pathmax-simply-isnt.html)
///
///    Programs which rely on libc for this functionality but do not attempt to canonicalize paths
///    will usually work if you change the working directory and use relative paths.
///  * The following lengths were considered too limiting to be enforced by this function:
///    * The UDF filesystem used on DVDs imposes a 1023-byte length limit on paths.
///    * When not using the `\?\` prefix to disable legacy compatibility, Windows paths  are
///      limited to 260 characters, which was arrived at as `A:\MAX_FILENAME_LENGTH<NULL>`.
///      [[5]](https://stackoverflow.com/a/1880453/435253)
///    * ISO 9660 without Joliet or Rock Ridge extensions does not permit periods in directory
///      names, directory trees more than 8 levels deep, or filenames longer than 32 characters.
///      [[6]](https://www.boost.org/doc/libs/1_36_0/libs/filesystem/doc/portability_guide.htm)
///
///  **TODO:**
///   * Validate that each path component is short enough to be valid.
///   * Write another function for enforcing the limits imposed by targeting optical media.
pub fn path_valid_portable<P: AsRef<Path> + ?Sized>(value: &P) -> Result<(), OsString> {
    #![allow(clippy::match_same_arms, clippy::decimal_literal_representation)]
    let path = value.as_ref();

    // TODO: Should I refuse incorrect Unicode normalization since Finder doesn't like it?
    // Source: https://news.ycombinator.com/item?id=16993687

    // TODO: Windows does not permit period as the last character.
    // Source: https://www.boost.org/doc/libs/1_36_0/libs/filesystem/doc/portability_guide.htm

    // TODO: Rework filename_valid_portable into an internal function which doesn't call this and
    //       then apply it to each path component.
    if path.as_os_str().len() > 32760 {
        // Limit length to fit on VFAT/exFAT when using the `\?\` prefix to disable legacy limits
        // Source: https://en.wikipedia.org/wiki/Comparison_of_file_systems
        Err(format!("Path is too long ({} chars): {:?}",
                    path.as_os_str().len(), path).into())
    } else if path.to_string_lossy().as_bytes().iter().any(|c| match c {
        // invalid on all APIs which don't use counted strings like inside the NT kernel
        b'\0' => true,
        // invalid under FAT*, VFAT, exFAT, and NTFS
        0x0..=0x1f | 0x7f | b'"' | b'*' | b'<' | b'>' | b'?' | b'|' => true,
        // let everything else through
        _ => false,
    }) {
        #[allow(clippy::use_debug)]
        Err(format!("Path contains invalid characters: {:?}", path).into())
    } else if path.as_os_str().is_empty() {
        Err("Path is empty".into())
    } else if let Some(file_stem) = path.file_stem() {
        // Reserved DOS filenames that still can't be used on modern Windows for compatibility
        let stem = file_stem.to_string_lossy().to_uppercase();
        if RESERVED_DOS_FILENAMES.iter().any(|&x| x == stem) {
            return Err(format!("Filename is reserved on Windows: {:?}", file_stem).into());
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

/// The string is a valid file/folder name on all major filesystems and OSes
///
/// ## Use For:
///  * Output file or directory names within a parent directory specified through other means.
///
/// ## Relevant Conventions:
///  * Most of the time, you want to let users specify a full path via `path_valid_portable`
///    instead.
///
/// ## Cautions:
///  * To ensure files can be copied/moved without issue, this validator may impose stricter
///    restrictions on filenames than your filesystem. Do *not* use it for input filenames.
///  * This validator cannot guarantee that a given filename will be valid once other
///    considerations such as overall path length limits are taken into account.
///  * As a more reliable validity check, you are advised to open a handle to the file in question
///    as early in your program's operation as possible and keep it open until you are finished.
///    This will both verify its validity and minimize the window in which another process could
///    render the path invalid.
///
/// ## Design Considerations: [[3]](https://en.wikipedia.org/wiki/Comparison_of_file_systems#Limits)
///  * In the interest of not inconveniencing users in the most common case, this validator imposes
///    a 255-character length limit.
///  * The eCryptFS home directory encryption offered by Ubuntu Linux imposes a 143-character
///    length limit when filename encryption is enabled.
///    [[4]](https://bugs.launchpad.net/ecryptfs/+bug/344878)
///  * the Joliet extensions for ISO 9660 are specified to support only 64-character filenames and
///    tested to support either 103 or 110 characters depending whether you ask the mkisofs
///    developers or Microsoft. [[5]](https://en.wikipedia.org/wiki/Joliet_(file_system))
///
/// **TODO:** Consider retiring this in favour of more specialized validators for filename
/// patterns, prefixes, and/or suffixes, to properly account for how "you can specify a name but
/// not a path" generally comes about.
pub fn filename_valid_portable<P: AsRef<Path> + ?Sized>(value: &P) -> Result<(), OsString> {
    #![allow(clippy::match_same_arms)]
    let path = value.as_ref();

    // Anything that's invalid in a path is invalid in a path component
    path_valid_portable(path)?;

    // TODO: Probably a good idea to disallow `.` and `..`

    if path.as_os_str().len() > 255 {
        Err(format!("File/folder name is too long ({} chars): {:?}",
                    path.as_os_str().len(), path).into())
    } else if path.to_string_lossy().as_bytes().iter().any(|c| match c {
        // POSIX path separator (invalid on Unixy platforms like Linux and BSD)
        b'/' => true,
        // HFS/Carbon path separator (invalid in filenames on MacOS and Mac filesystems)
        // DOS/Win32 drive separator (invalid in filenames on Windows and Windows filesystems)
        b':' => true,
        // DOS/Windows path separator (invalid in filenames on Windows and Windows filesystems)
        b'\\' => true,
        // Let everything else through
        _ => false,
    }) {
        #[allow(clippy::use_debug)]
        Err(format!("File/folder names cannot contain path separators: {:?}", path).into())
    } else {
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use super::*;

    // ---- path_readable ----

    // TODO: Use a `cfg` to pick some appropriate alternative paths for Windows
    #[test]
    fn path_readable_basic_functionality() {
        // Existing paths
        assert!(path_readable(OsStr::new("/")).is_ok());                       // OK Folder
        assert!(path_readable(OsStr::new("/etc/passwd")).is_ok());             // OK File
        assert!(path_readable(OsStr::new("/bin/../etc/.././.")).is_ok());      // Not canonicalized
        assert!(path_readable(OsStr::new("/../../../..")).is_ok());            // Above root

        // Inaccessible, nonexistent, or invalid paths
        assert!(path_readable(OsStr::new("")).is_err());                       // Empty String
        assert!(path_readable(OsStr::new("/etc/shadow")).is_err());            // Denied File
        assert!(path_readable(OsStr::new("/etc/ssl/private")).is_err());       // Denied Folder
        assert!(path_readable(OsStr::new("/nonexistant_test_path")).is_err()); // Missing Path
        assert!(path_readable(OsStr::new("/null\0containing")).is_err());      // Invalid CString

    }

    #[cfg(not(windows))]
    #[test]
    fn path_readable_invalid_utf8() {
        assert!(path_readable(OsStr::from_bytes(b"/not\xffutf8")).is_err());   // Invalid UTF-8
        // TODO: Non-UTF8 path that actually IS valid
    }

    // TODO: #[cfg(windows) test with un-paired UTF-16 surrogates

    // ---- filename_valid_portable ----

    const VALID_FILENAMES: &[&str] = &[
        // regular, space, leading, and trailing periods
        "test1", "te st", ".test", "test.",
        // Stuff which would break if the DOS reserved names check is doing dumb pattern matching
        "lpt", "lpt0", "lpt10",
    ];

    const PATHS_WITH_SEPARATORS: &[&str] = &[
        "t:est\\sss", // DOS drive separator
        "te\\stssss", // DOS path separator
        "te/stsssss", // POSIX path separator

        // Absolute paths
        "\\\\lo\\ca", // UNC
        "\\te\\stss", // DOS path separator
        "/te/stssss", // POSIX path separator
    ];

    const INVALID_PORTABLE_FILENAMES: &[&str] = &[
        "test\x03", "test\x07", "test\x08", "test\x0B", "test\x7f",  // Control characters (VFAT)
        "\"test\"", "<testsss", "testsss>", "testsss|", "testsss*", "testsss?", "?estsss", // VFAT
        "CON", "Con", "coN", "cOn", "CoN", "con", "lpt1", "com9", // DOS/Win32 API
        "", "\0"]; // POSIX

    #[test]
    fn filename_valid_portable_accepts_valid_names() {
        for path in VALID_FILENAMES {
            assert!(filename_valid_portable(OsStr::new(path)).is_ok(), "{:?}", path);
        }
    }

    #[test]
    fn filename_valid_portable_refuses_path_separators() {
        for path in PATHS_WITH_SEPARATORS {
            assert!(filename_valid_portable(OsStr::new(path)).is_err(), "{:?}", path);
        }
    }

    #[test]
    fn filename_valid_portable_refuses_invalid_characters() {
        for fname in INVALID_PORTABLE_FILENAMES {
            assert!(filename_valid_portable(OsStr::new(fname)).is_err(), "{:?}", fname);
        }
    }

    #[test]
    fn filename_valid_portable_enforces_length_limits() {
        // 256 characters
        let mut test_str = std::str::from_utf8(&[b'X'; 256]).expect("parsing constant");
        assert!(filename_valid_portable(OsStr::new(test_str)).is_err());

        // 255 characters (maximum for NTFS, ext2/3/4, and a lot of others)
        test_str = std::str::from_utf8(&[b'X'; 255]).expect("parsing constant");
        assert!(filename_valid_portable(OsStr::new(test_str)).is_ok());
    }

    #[cfg(not(windows))]
    #[test]
    fn filename_valid_portable_accepts_valid_but_malformed_names() {
        // Ensure that we don't refuse invalid UTF-8 that "bag of bytes" POSIX allows
        assert!(filename_valid_portable(OsStr::from_bytes(b"\xff")).is_ok());
    }

    // TODO: #[cfg(windows) test with un-paired UTF-16 surrogates

    // ---- path_valid_portable ----

    #[test]
    fn path_valid_portable_accepts_valid_names() {
        for path in VALID_FILENAMES {
            assert!(path_valid_portable(OsStr::new(path)).is_ok(), "{:?}", path);
        }
    }

    #[test]
    fn path_valid_portable_accepts_path_separators() {
        for path in PATHS_WITH_SEPARATORS {
            assert!(path_valid_portable(OsStr::new(path)).is_ok(), "{:?}", path);
        }
    }

    #[test]
    fn path_valid_portable_refuses_invalid_characters() {
        for fname in INVALID_PORTABLE_FILENAMES {
            assert!(path_valid_portable(OsStr::new(fname)).is_err(), "{:?}", fname);
        }
    }

    #[test]
    fn path_valid_portable_enforces_length_limits() {
        // 32761 characters
        let mut test_str = std::str::from_utf8(&[b'X'; 32761]).expect("parsing constant");
        assert!(path_valid_portable(OsStr::new(test_str)).is_err());

        // 32760 characters (maximum for FAT32/VFAT/exFAT)
        test_str = std::str::from_utf8(&[b'X'; 32760]).expect("parsing constant");
        assert!(path_valid_portable(OsStr::new(test_str)).is_ok());
    }

    #[cfg(not(windows))]
    #[test]
    fn path_valid_portable_accepts_valid_but_malformed_names() {
        // Ensure that we don't refuse invalid UTF-8 that "bag of bytes" POSIX allows
        assert!(path_valid_portable(OsStr::from_bytes(b"/\xff/foo")).is_ok());
    }

    // TODO: #[cfg(windows) test with un-paired UTF-16 surrogates
}
