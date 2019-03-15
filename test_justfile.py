#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Simple harness for automating the task of testing that the justfile works
reasonably properly after a refactoring.
"""

from __future__ import (absolute_import, division, print_function,
                        with_statement, unicode_literals)

__author__ = "Stephan Sokolow (deitarion/SSokolow)"
__appname__ = "Test harness for justfile"
__version__ = "0.1"
__license__ = "MIT or Apache 2.0"

import logging, os, re, shutil, subprocess, unittest
log = logging.getLogger(__name__)

# TODO: Extract this from the justfile rather than hard-coding it
CARGO_BUILD_TARGET = "i686-unknown-linux-musl"

class TestJustfile(unittest.TestCase):
    """Test suite for rust-cli-boilerplate justfile"""
    outpath = "target/{}/release/boilerplate".format(CARGO_BUILD_TARGET)

    def _assert_task(self, task, regex):
        """Helper to avoid duplication-related typos

        (Checks both the exit code and the output printed)
        """
        self.assertRegex(
            subprocess.check_output(['just'] + task, stderr=subprocess.STDOUT),
            regex)

    def test_bloat(self):
        """just bloat"""
        self._assert_task(['bloat'], b'Crate Name\n')
        self._assert_task(['bloat', '--', '--crates'], b'Size Name\n')

    def test_build(self):
        """just build"""
        if os.path.exists(self.outpath):
            os.remove(self.outpath)

        self._assert_task(['build'], br'Finished release \[optimized\]')
        self.assertTrue(os.path.isfile(self.outpath))

    def test_build_release(self):
        """just build-dist"""
        for ext in ('', '.stripped', '.packed'):
            if os.path.exists(self.outpath):
                os.remove(self.outpath + ext)

        self._assert_task(['build-dist', '--set', 'upx_flags', ''],
                          b'--== Final Result ==--')

        for ext in ('', '.stripped', '.packed'):
            self.assertTrue(os.path.isfile(self.outpath + ext))

    def test_check(self):
        """just check"""
        self._assert_task(['check'], br'Finished release \[optimized\]')
        self._assert_task(['check', '--', '--message-format', 'json'],
                          br'\s*"target"\s*:\s*{')

    def test_dist(self):
        """just dist"""
        outpath = 'dist/{}'.format(os.path.basename(self.outpath))
        if os.path.exists(outpath):
            os.remove(outpath)

        self._assert_task(['dist', '--set', 'upx_flags', ''],
                          br'Finished release \[optimized\]')
        self.assertTrue(os.path.isfile(outpath))

    def test_dist_supplemental(self):
        """just dist-supplemental"""
        self._assert_task(['dist-supplemental'],
                          br'Finished release \[optimized\]')
        # TODO: Remove build artifact before and test for it after
        # TODO: Test that help2man actually generated a valid manpage
        # TODO: Test for some distinctive fragment in each completion script

    def test_doc(self):
        """just doc"""
        for command, expected in (
                (['doc'], br' Finished release \[optimized\]'),
                (['doc', '--', '--message-format', 'json'],
                 br'\s*"target"\s*:\s*{')):

            # Save time by trusting that, if `cargo doc` regenerates
            # part of the docs, it's indicative of full proper function
            outpath = "target/{}/doc/log/index.html".format(CARGO_BUILD_TARGET)
            if os.path.exists(outpath):
                os.remove(outpath)

            self._assert_task(command, expected)
            self.assertTrue(os.path.isfile(outpath),
                '%s not a file (%s)' % (outpath, command))

    def test_fmt_check(self):
        """just fmt-check"""
        self._assert_task(['fmt-check'],
            b'\n(' + re.escape(b'\x1b[m\x0f\x1b[31m\x1b[1m') + b')?' +
            br'warning: (' + re.escape(b'\x1b[m\x0f\x1b[1m') +
            br')?found TODO')
        self._assert_task(['fmt-check', '--', '-V'],
                          br'\nrustfmt \S+-nightly \(\S+ 2\d\d\d-\d\d-\d\d\)')

    # TODO: The following commands need to be tested in a less destructive way
    # - clean +args=''
    # - fmt +args=''
    # TODO: Assert that echo'd kcachegrind command didn't contain --release

    def test_kcov(self):
        """just kcov"""
        outdir = 'target/kcov/html'

        if os.path.exists(outdir):
            shutil.rmtree(outdir)

        self._assert_task(['kcov'], br'\ntest result:')
        # TODO: Assert that echo'd kcov command didn't contain --release
        self.assertTrue(os.path.isdir(outdir))

    def test_run(self):
        """just run"""
        # TODO: Test argument-less operation by verifying that the failure
        #       is due to the called binary panicking at unimplemented!
        self._assert_task(['run', '--', '--help'], br'\nUSAGE:')

    def test_test(self):
        """just test (and the default command)"""
        for subcommand in ([], ['test']):
            self._assert_task(subcommand, br'\ntest result: ')

"""
    install # Install the binary, shell completions, and a help file
    install-cargo-deps # `install-rustup-deps` and then `cargo install` tools
    install-rustup-deps # Install (don't update) nightly `channel` toolchains, plus `CARGO_BUILD_TARGET`, clippy, and rustfmt
    uninstall # Remove any files installed by the `install` task (but leave any parent directories created)
"""

if __name__ == '__main__':
    print("Commands I'm reticent to auto-test in an open-source script "
          "because they'll modify things outside the project directory "
          "and are only safe to run automatically in certain circumstances:\n"
          "\t install, install-cargo-deps, install-rustup-deps, uninstall\n")

    print("""Commands which can't be auto-tested as currently written:
    install-apt-deps      (Because it calls sudo)
    install-deps          (Because it depends on install-apt-deps)
    kcachegrind +args=''  (Because it opens KCachegrind for interactive use)
    """)

    unittest.main()


# vim: set sw=4 sts=4 expandtab :
