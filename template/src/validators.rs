/*! Validator functions suitable for use with `Clap` and `StructOpt` */
// Copyright 2017-2020, Stephan Sokolow

use std::ffi::OsString;
use std::fs::File;
use std::path::{Component, Path};

use faccess::PathExt;

/// Special filenames which cannot be used for real files under Win32
///
/// (Unless your app uses the `\\?\` path prefix to bypass legacy Win32 API compatibility
/// limitations)
///
/// **NOTE:** These are still reserved if you append an extension to them.
///
/// Sources:
/// * [Boost Path Name Portability Guide
/// ](https://www.boost.org/doc/libs/1_36_0/libs/filesystem/doc/portability_guide.htm)
/// * Wikipedia: [Filename: Comparison of filename limitations
/// ](https://en.wikipedia.org/wiki/Filename#Comparison_of_filename_limitations)
///
/// **TODO:** Decide what (if anything) to do about the NTFS "only in root directory" reservations.
#[rustfmt::skip]
pub const RESERVED_DOS_FILENAMES: &[&str] = &["AUX", "CON", "NUL", "PRN",   // Comments for rustfmt
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", // Serial Ports
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9", // Parallel Ports
    "CLOCK$", "$IDLE$", "CONFIG$", "KEYBD$", "LST", "SCREEN$"];

/// Test that the given path *should* be writable
///
/// **TODO:** Implement Windows tests for this.
#[allow(dead_code)] // TEMPLATE:REMOVE
#[cfg(unix)]
pub fn path_output_dir<P: AsRef<Path> + ?Sized>(value: &P) -> Result<(), OsString> {
    let path = value.as_ref();

    if !path.is_dir() {
        return Err(format!("Not a directory: {}", path.display()).into());
    }

    if path.writable() {
        return Ok(());
    }

    Err(format!("Would be unable to write to destination directory: {}", path.display()).into())
}

/// The given path is a file that can be opened for reading
///
/// ## Use For:
///  * Input file paths
///
/// ## Relevant Conventions:
///  * Commands which read from `stdin` by default should use `-f` to specify the input path.
///    [[1]](http://www.catb.org/esr/writings/taoup/html/ch10s05.html)
///  * Commands which read from files by default should use positional arguments to specify input
///    paths.
///  * Allow an arbitrary number of input paths if feasible.
///  * Interpret a value of `-` to mean "read from `stdin`" if feasible.
///    [[2]](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html)
///
/// **TODO:** Provide an alternative variant of this which accepts `-` regardless of whether a file
/// of that name exists.
///
/// **Note:** The following command-lines, which interleave files and `stdin`, are a good test of
/// how the above conventions should interact:
///
///     data_source | my_utility_a header.dat - footer.dat > output.dat
///     data_source | my_utility_b -f header.dat -f - -f footer.dat > output.dat
///
/// ## Cautions:
///  * This will momentarily open the given path for reading to verify that it is readable.
///    However, relying on this to remain true will introduce a race condition. This validator is
///    intended only to allow your program to exit as quickly as possible in the case of obviously
///    bad input.
///  * As a more reliable validity check, you are advised to open a handle to the file in question
///    as early in your program's operation as possible, use it for all your interactions with the
///    file, and keep it open until you are finished. This will both verify its validity and
///    minimize the window in which another process could render the path invalid.
#[allow(dead_code)] // TEMPLATE:REMOVE
#[rustfmt::skip]
pub fn path_readable_file<P: AsRef<Path> + ?Sized>(value: &P)
        -> std::result::Result<(), OsString> {
    let path = value.as_ref();

    if path.is_dir() {
        return Err(format!("{}: Input path must be a file, not a directory",
                           path.display()).into());
    }

    File::open(path).map(|_| ()).map_err(|e| format!("{}: {}", path.display(), e).into())
}

// TODO: Implement path_readable_dir and path_readable for --recurse use-cases

/// The given path is valid on all major filesystems and OSes
///
/// ## Use For:
///  * Output file or directory paths
///
/// ## Relevant Conventions:
///  * Use `-o` to specify the output path if doing so is optional.
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
/// ## Design Considerations:
///  * Many popular Linux filesystems impose no total length limit.
///  * This function imposes a 32,760-character limit for compatibility with flash drives formatted
///    FAT32 or exFAT. [[4]](https://en.wikipedia.org/wiki/Comparison_of_file_systems#Limits)
///  * Some POSIX API functions, such as `getcwd()` and `realpath()` rely on the `PATH_MAX`
///    constant, which typically specifies a length of 4096 bytes including terminal `NUL`, but
///    this is not enforced by the filesystem itself.
///    [[5]](https://insanecoding.blogspot.com/2007/11/pathmax-simply-isnt.html)
///
///    Programs which rely on libc for this functionality but do not attempt to canonicalize paths
///    will usually work if you change the working directory and use relative paths.
///  * The following lengths were considered too limiting to be enforced by this function:
///    * The UDF filesystem used on DVDs imposes a 1023-byte length limit on paths.
///    * When not using the `\\?\` prefix to disable legacy compatibility, Windows paths  are
///      limited to 260 characters, which was arrived at as `A:\MAX_FILENAME_LENGTH<NULL>`.
///      [[6]](https://stackoverflow.com/a/1880453/435253)
///    * ISO 9660 without Joliet or Rock Ridge extensions does not permit periods in directory
///      names, directory trees more than 8 levels deep, or filenames longer than 32 characters.
///      [[7]](https://www.boost.org/doc/libs/1_36_0/libs/filesystem/doc/portability_guide.htm)
///
///  **TODO:**
///   * Write another function for enforcing the limits imposed by targeting optical media.
#[allow(dead_code)] // TEMPLATE:REMOVE
pub fn path_valid_portable<P: AsRef<Path> + ?Sized>(value: &P) -> Result<(), OsString> {
    let path = value.as_ref();

    #[allow(clippy::decimal_literal_representation)] // Path lengths are most intuitive as decimal
    if path.as_os_str().is_empty() {
        Err("Path is empty".into())
    } else if path.as_os_str().len() > 32760 {
        // Limit length to fit on VFAT/exFAT when using the `\\?\` prefix to disable legacy limits
        // Source: https://en.wikipedia.org/wiki/Comparison_of_file_systems
        Err(format!("Path is too long ({} chars): {:?}", path.as_os_str().len(), path).into())
    } else {
        for component in path.components() {
            if let Component::Normal(string) = component {
                filename_valid_portable(string)?
            }
        }
        Ok(())
    }
}

/// The string is a valid file/folder name on all major filesystems and OSes
///
/// ## Use For:
///  * Output file or directory names within a parent directory specified through other means.
///
/// ## Relevant Conventions:
///  * Most of the time, you want to let users specify a full path via [`path_valid_portable`
///    ](fn.path_valid_portable.html)instead.
///
/// ## Cautions:
///  * To ensure files can be copied/moved without issue, this validator may impose stricter
///    restrictions on filenames than your filesystem. Do *not* use it for input filenames.
///  * This validator cannot guarantee that a given filename will be valid once other
///    considerations such as overall path length limits are taken into account.
///  * As a more reliable validity check, you are advised to open a handle to the file in question
///    as early in your program's operation as possible, use it for all your interactions with the
///    file, and keep it open until you are finished. This will both verify its validity and
///    minimize the window in which another process could render the path invalid.
///
/// ## Design Considerations:
///  * In the interest of not inconveniencing users in the most common case, this validator imposes
///    a 255-character length limit.
///    [[1]](https://en.wikipedia.org/wiki/Comparison_of_file_systems#Limits)
///  * The eCryptFS home directory encryption offered by Ubuntu Linux imposes a 143-character
///    length limit when filename encryption is enabled.
///    [[2]](https://bugs.launchpad.net/ecryptfs/+bug/344878)
///  * the Joliet extensions for ISO 9660 are specified to support only 64-character filenames and
///    tested to support either 103 or 110 characters depending whether you ask the mkisofs
///    developers or Microsoft. [[3]](https://en.wikipedia.org/wiki/Joliet_(file_system))
///  * The [POSIX Portable Filename Character Set
///    ](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_282)
///    is too restrictive to be baked into a general-purpose validator.
///
/// **TODO:** Consider converting this to a private function that just exists as a helper for the
/// path validator in favour of more specialized validators for filename patterns, prefixes, and/or
/// suffixes, to properly account for how "you can specify a name but not a path" generally
/// comes about.
#[allow(dead_code)] // TEMPLATE:REMOVE
pub fn filename_valid_portable<P: AsRef<Path> + ?Sized>(value: &P) -> Result<(), OsString> {
    let path = value.as_ref();

    // TODO: Should I refuse incorrect Unicode normalization since Finder doesn't like it or just
    //       advise users to run a normalization pass?
    // Source: https://news.ycombinator.com/item?id=16993687

    // Check that the length is within range
    let os_str = path.as_os_str();
    if os_str.len() > 255 {
        #[rustfmt::skip]
        return Err(format!("File/folder name is too long ({} chars): {}",
                           path.as_os_str().len(), path.display()).into());
    }

    // Check for invalid characters
    let lossy_str = match os_str.to_str() {
        Some(string) => string,
        None => {
            return Err("File/folder names containing non-UTF8 characters aren't portable".into())
        },
    };
    let last_char = match lossy_str.chars().last() {
        Some(chr) => chr,
        None => return Err("File/folder name is empty".into()),
    };
    if [' ', '.'].iter().any(|&x| x == last_char) {
        // The Windows shell and UI don't support component names ending in periods or spaces
        // Source: https://docs.microsoft.com/en-us/windows/desktop/FileIO/naming-a-file
        return Err("Windows forbids path components ending with spaces/periods".into());
    }

    #[allow(clippy::match_same_arms)] // Would need to cram everything onto one arm otherwise
    if lossy_str.as_bytes().iter().any(|c| match c {
        // invalid on all APIs which don't use counted strings like inside the NT kernel
        b'\0' => true,
        // invalid under FAT*, VFAT, exFAT, and NTFS
        0x1..=0x1f | 0x7f | b'"' | b'*' | b'<' | b'>' | b'?' | b'|' => true,
        // POSIX path separator (invalid on Unixy platforms like Linux and BSD)
        b'/' => true,
        // HFS/Carbon path separator (invalid in filenames on MacOS and Mac filesystems)
        // DOS/Win32 drive separator (invalid in filenames on Windows and Windows filesystems)
        b':' => true,
        // DOS/Windows path separator (invalid in filenames on Windows and Windows filesystems)
        b'\\' => true,
        // let everything else through
        _ => false,
    }) {
        #[rustfmt::skip]
        return Err(format!("Path component contains invalid characters: {}",
                path.display()).into());
    }

    // Reserved DOS filenames that still can't be used on modern Windows for compatibility
    if let Some(file_stem) = path.file_stem() {
        let stem = file_stem.to_string_lossy().to_uppercase();
        if RESERVED_DOS_FILENAMES.iter().any(|&x| x == stem) {
            Err(format!("Filename is reserved on Windows: {:?}", file_stem).into())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports, clippy::panic, clippy::result_expect_used)] // OK for tests

    use super::*;
    use std::ffi::OsStr;

    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStringExt;

    #[test]
    #[cfg(unix)]
    #[rustfmt::skip]
    fn path_output_dir_basic_functionality() {
        assert!(path_output_dir(OsStr::new("/")).is_err());                      // Root
        assert!(path_output_dir(OsStr::new("/tmp")).is_ok());                    // OK Folder
        assert!(path_output_dir(OsStr::new("/dev/null")).is_err());              // OK File
        assert!(path_output_dir(OsStr::new("/etc/shadow")).is_err());            // Denied File
        assert!(path_output_dir(OsStr::new("/etc/ssl/private")).is_err());       // Denied Folder
        assert!(path_output_dir(OsStr::new("/nonexistant_test_path")).is_err()); // Missing Path
        assert!(path_output_dir(OsStr::new("/tmp\0with\0null")).is_err());       // Invalid CString
        // TODO: Not-already-canonicalized paths (eg. relative paths)

        assert!(path_output_dir(OsStr::from_bytes(b"/not\xffutf8")).is_err());   // Invalid UTF-8
        // TODO: Non-UTF8 path that actually does exist and is writable
    }

    #[test]
    #[cfg(windows)]
    fn path_output_dir_basic_functionality() {
        unimplemented!("TODO: Implement Windows version of path_output_dir");
    }

    // ---- path_readable_file ----

    #[cfg(unix)]
    #[test]
    #[rustfmt::skip]
    fn path_readable_file_basic_functionality() {
        // Existing paths
        assert!(path_readable_file(OsStr::new("/bin/sh")).is_ok());                 // OK File
        assert!(path_readable_file(OsStr::new("/bin/../etc/.././bin/sh")).is_ok()); // Non-canonic.
        assert!(path_readable_file(OsStr::new("/../../../../bin/sh")).is_ok());     // Above root

        // Inaccessible, nonexistent, or invalid paths
        assert!(path_readable_file(OsStr::new("")).is_err());                       // Empty String
        assert!(path_readable_file(OsStr::new("/")).is_err());                      // OK Folder
        assert!(path_readable_file(OsStr::new("/etc/shadow")).is_err());            // Denied File
        assert!(path_readable_file(OsStr::new("/etc/ssl/private")).is_err());       // Denied Foldr
        assert!(path_readable_file(OsStr::new("/nonexistant_test_path")).is_err()); // Missing Path
        assert!(path_readable_file(OsStr::new("/null\0containing")).is_err());      // Invalid CStr
    }

    #[cfg(windows)]
    #[test]
    fn path_readable_file_basic_functionality() {
        unimplemented!("TODO: Pick some appropriate equivalent test paths for Windows");
    }

    #[cfg(unix)]
    #[test]
    #[rustfmt::skip]
    fn path_readable_file_invalid_utf8() {
        assert!(path_readable_file(OsStr::from_bytes(b"/not\xffutf8")).is_err()); // Invalid UTF-8
        // TODO: Non-UTF8 path that actually IS valid
    }
    #[cfg(windows)]
    #[test]
    #[rustfmt::skip]
    fn path_readable_file_unpaired_surrogates() {
        assert!(path_readable_file(&OsString::from_wide(
            &['C' as u16, ':' as u16, '\\' as u16, 0xd800])).is_err());
        // TODO: Unpaired surrogate path that actually IS valid
    }

    // ---- filename_valid_portable ----

    #[rustfmt::skip]
    const VALID_FILENAMES: &[&str] = &[
        "-",                       // stdin/stdout
        "test1", "te st", ".test", // regular, space, and leading period
        "lpt", "lpt0", "lpt10",    // would break if DOS reserved check is doing dumb matching
    ];

    // Paths which should pass because std::path::Path will recognize the separators
    // TODO: Actually run the tests on Windows to make sure they work
    #[cfg(windows)]
    const PATHS_WITH_NATIVE_SEPARATORS: &[&str] =
        &["re/lative", "/ab/solute", "re\\lative", "\\ab\\solute"];
    #[cfg(unix)]
    const PATHS_WITH_NATIVE_SEPARATORS: &[&str] = &["re/lative", "/ab/solute"];

    // Paths which should fail because std::path::Path won't recognize the separators and we don't
    // want them showing up in the components.
    #[cfg(windows)]
    const PATHS_WITH_FOREIGN_SEPARATORS: &[&str] = &["Classic Mac HD:Folder Name:File"];
    #[cfg(unix)]
    const PATHS_WITH_FOREIGN_SEPARATORS: &[&str] = &[
        "relative\\win32",
        "C:\\absolute\\win32",
        "\\drive\\relative\\win32",
        "\\\\unc\\path\\for\\win32",
        "Classic Mac HD:Folder Name:File",
    ];

    // Source: https://docs.microsoft.com/en-us/windows/desktop/FileIO/naming-a-file
    #[rustfmt::skip]
    const INVALID_PORTABLE_FILENAMES: &[&str] = &[
        "test\x03", "test\x07", "test\x08", "test\x0B", "test\x7f",  // Control characters (VFAT)
        "\"test\"", "<testsss", "testsss>", "testsss|", "testsss*", "testsss?", "?estsss", // VFAT
        "ends with space ", "ends_with_period.", // DOS/Win32
        "CON", "Con", "coN", "cOn", "CoN", "con", "lpt1", "com9", // Reserved names (DOS/Win32)
        "con.txt", "lpt1.dat", // DOS/Win32 API (Reserved names are extension agnostic)
        "", "\0"]; // POSIX

    #[test]
    fn filename_valid_portable_accepts_valid_names() {
        for path in VALID_FILENAMES {
            assert!(filename_valid_portable(OsStr::new(path)).is_ok(), "{:?}", path);
        }
    }

    #[test]
    fn filename_valid_portable_refuses_path_separators() {
        for path in PATHS_WITH_NATIVE_SEPARATORS {
            assert!(filename_valid_portable(OsStr::new(path)).is_err(), "{:?}", path);
        }
        for path in PATHS_WITH_FOREIGN_SEPARATORS {
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
    fn filename_valid_portable_refuses_empty_strings() {
        assert!(filename_valid_portable(OsStr::new("")).is_err());
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

    #[cfg(unix)]
    #[test]
    fn filename_valid_portable_refuses_non_utf8_bytes() {
        // Ensure that we refuse invalid UTF-8 since it's unclear if/how things like POSIX's
        // "bag of bytes" paths and Windows's un-paired UTF-16 surrogates map to each other.
        assert!(filename_valid_portable(OsStr::from_bytes(b"\xff")).is_err());
    }
    #[cfg(windows)]
    #[test]
    fn filename_valid_portable_accepts_unpaired_surrogates() {
        assert!(path_valid_portable(&OsString::from_wide(&[0xd800])).is_ok());
    }

    // ---- path_valid_portable ----

    #[test]
    fn path_valid_portable_accepts_valid_names() {
        for path in VALID_FILENAMES {
            assert!(path_valid_portable(OsStr::new(path)).is_ok(), "{:?}", path);
        }

        // No filename (.file_stem() returns None)
        assert!(path_valid_portable(OsStr::new("foo/..")).is_ok());
    }

    #[test]
    fn path_valid_portable_accepts_native_path_separators() {
        for path in PATHS_WITH_NATIVE_SEPARATORS {
            assert!(path_valid_portable(OsStr::new(path)).is_ok(), "{:?}", path);
        }

        // Verify that repeated separators are getting collapsed before filename_valid_portable
        // sees them.
        // TODO: Make this conditional on platform and also test repeated backslashes on Windows
        assert!(path_valid_portable(OsStr::new("/path//with/repeated//separators")).is_ok());
    }

    #[test]
    fn path_valid_portable_refuses_foreign_path_separators() {
        for path in PATHS_WITH_FOREIGN_SEPARATORS {
            assert!(path_valid_portable(OsStr::new(path)).is_err(), "{:?}", path);
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
        let mut test_string = String::with_capacity(255 * 130);
        #[allow(clippy::decimal_literal_representation)] // Path lengths more intuitive as decimal
        while test_string.len() < 32761 {
            test_string.push_str(std::str::from_utf8(&[b'X'; 255]).expect("utf8 from literal"));
            test_string.push('/');
        }

        // >32760 characters
        assert!(path_valid_portable(OsStr::new(&test_string)).is_err());

        // 32760 characters (maximum for FAT32/VFAT/exFAT)
        test_string.truncate(32760);
        assert!(path_valid_portable(OsStr::new(&test_string)).is_ok());

        // 256 characters with no path separators
        test_string.truncate(255);
        test_string.push('X');
        assert!(path_valid_portable(OsStr::new(&test_string)).is_err());

        // 255 characters with no path separators
        test_string.truncate(255);
        assert!(path_valid_portable(OsStr::new(&test_string)).is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn path_valid_portable_accepts_non_utf8_bytes() {
        // Ensure that we refuse invalid UTF-8 since it's unclear if/how things like POSIX's
        // "bag of bytes" paths and Windows's un-paired UTF-16 surrogates map to each other.
        assert!(path_valid_portable(OsStr::from_bytes(b"/\xff/foo")).is_err());
    }
    #[cfg(windows)]
    #[test]
    fn path_valid_portable_accepts_unpaired_surrogates() {
        #[rustfmt::skip]
        assert!(path_valid_portable(&OsString::from_wide(
                    &['C' as u16, ':' as u16, '\\' as u16, 0xd800])).is_ok());
    }
}
