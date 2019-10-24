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
__license__ = "Apache-2.0 OR MIT"

import logging, os, re, shutil, subprocess, unittest
from gzip import GzipFile

from gen_justfile_reference import get_evaluated_variables

log = logging.getLogger(__name__)


class TestJustfile(unittest.TestCase):
    """Test suite for rust-cli-boilerplate justfile"""

    def __init__(self, methodName='runTest'):
        super(TestJustfile, self).__init__(methodName=methodName)
        self.vars = get_evaluated_variables(include_private=True)

    def _assert_task(self, task, regex):
        """Run a just task and assert the exit code and output printed"""
        output = subprocess.check_output(['just'] + task,
                                         stderr=subprocess.STDOUT)
        self.assertRegex(output, regex)
        return output

    def _assert_file_contains(self, path, substr, count=None):
        """Check that a given file contains a string `count` times"""
        opener = GzipFile if os.path.splitext(path)[1] == '.gz' else open
        with opener(path) as fobj:
            found = fobj.read().count(substr)

            # Quick hack to support "any number is OK"
            if count is None:
                count = found = min(found, 1)
                count_str = "at least 1"
            else:
                count_str = count

            self.assertEqual(count, found, "Expected %s occurrence(s) of %r "
                             "(got %s)" % (count_str, substr, found))

    def test_a_invariants(self):
        """invariants required by tests

        (The _a_ in the name is just to make it run first)
        """
        variables = get_evaluated_variables(include_private=True)

        self.assertNotIn('--release', variables['_build_flags'],
            "--release shouldn't be in the default build flags, since they "
            "get used by dev-mode commands.")

    def test_bloat(self):
        """just bloat"""
        self._assert_task(['bloat'], b'Crate Name\n')
        self._assert_task(['bloat', '--', '--crates'], b'Size Name\n')

    def test_build(self):
        """just build"""
        if os.path.exists(self.vars['_dbg_bin_path']):
            os.remove(self.vars['_dbg_bin_path'])

        self._assert_task(['build'], br'Finished dev \[unoptimized')
        self.assertTrue(os.path.isfile(self.vars['_dbg_bin_path']))

    def test_build_release(self):
        """just build-dist"""
        for ext in ('', '.stripped', '.packed'):
            if os.path.exists(self.vars['_rls_bin_path']):
                os.remove(self.vars['_rls_bin_path'] + ext)

        self._assert_task(['build-dist', '--set', 'upx_flags', ''],
                          b'--== Final Result ==--')

        for ext in ('', '.stripped', '.packed'):
            self.assertTrue(os.path.isfile(self.vars['_rls_bin_path'] + ext))

    def test_check(self):
        """just check"""
        self._assert_task(['check'], br'Finished dev \[unoptimized')
        self._assert_task(['check', '--', '--message-format', 'json'],
                          br'\s*"target"\s*:\s*{')

    def test_clean(self):
        """just clean

        NOTE: Overrides _cargo to test what matters quickly.
        """
        self._assert_task(['clean', '--set', '_cargo', 'echo'],
            b'echo clean -v \nclean -v\n'
            b'export CARGO_TARGET_DIR="target/kcov" && echo clean -v\n'
            b'clean -v\nrm -rf dist\n')
        self._assert_task(['clean', '--set', '_cargo',
                           'echo', '--', '--release'],
            b'echo clean -v --release\nclean -v --release\n'
            b'export CARGO_TARGET_DIR="target/kcov" && '
            b'echo clean -v\n'
            b'clean -v\nrm -rf dist\n')

    def test_dist(self):
        """just dist"""
        outpath = 'dist/{}'.format(self.vars['_pkgname'])
        if os.path.exists(outpath):
            os.remove(outpath)

        self._assert_task(['dist', '--set', 'upx_flags', ''],
                          br'Finished release \[optimized\]')
        self.assertTrue(os.path.isfile(outpath))

    def test_dist_supplemental(self):
        """just dist-supplemental"""
        artifacts = ['boilerplate.1.gz', 'boilerplate.bash', 'boilerplate.zsh',
                     'boilerplate.elvish', 'boilerplate.powershell',
                     'boilerplate.fish']
        for fname in artifacts:
            if os.path.exists('dist/' + fname):
                os.remove('dist/' + fname)

        self._assert_task(['dist-supplemental'],
                          br'Finished release \[optimized\]')

        for fname in artifacts:
            self.assertTrue(os.path.isfile('dist/' + fname))

        # Trust that help2man and clap will do their own testing and just
        # verify that we're successfully invoking the proper functionality
        # (count=1 on the manpage to account for how help2man fails
        #  if you accidentally include --help in the base command)
        self._assert_file_contains('dist/boilerplate.1.gz',
                                   b'\n.SS "USAGE:"\n', count=1)
        self._assert_file_contains('dist/boilerplate.bash',
                                   'COMPREPLY=()')
        self._assert_file_contains('dist/boilerplate.elvish',
                                   'edit:complex-candidate')
        self._assert_file_contains('dist/boilerplate.fish',
                                   '__fish_use_subcommand')
        self._assert_file_contains('dist/boilerplate.powershell',
                                   '[CompletionResult]::new')
        self._assert_file_contains('dist/boilerplate.zsh',
                                   'typeset -A opt_args')

    def test_doc(self):
        """just doc"""
        for command, expected in (
                (['doc'], br' Finished dev \[unoptimized'),
                (['doc', '--', '--message-format', 'json'],
                 br'\s*"target"\s*:\s*{')):

            # Save time by trusting that, if `cargo doc` regenerates
            # part of the docs, it's indicative of full proper function
            outpath = ("target/{}/doc/log/index.html"
                       .format(self.vars['CARGO_BUILD_TARGET']))
            if os.path.exists(outpath):
                os.remove(outpath)

            self._assert_task(command, expected)
            self.assertTrue(os.path.isfile(outpath),
                '%s not a file (%s)' % (outpath, command))

    def test_fmt(self):
        """just fmt"""
        # Avoid having to save and restore the un-formatted versions by only
        # testing that the expected command gets emitted without error
        self._assert_task(['fmt', '--set', '_cargo_cmd', 'echo'],
                          b'(^|\n)echo [+]nightly fmt --\s*(\n|$)')
        self._assert_task(['fmt', '--set', '_cargo_cmd', 'echo', '--', '-V'],
                          b'(^|\n)echo [+]nightly fmt -- -V\s*(\n|$)')
        # TODO: Decide how to actually test that the files would be modified

    def test_fmt_check(self):
        """just fmt-check"""
        self._assert_task(['fmt-check'],
            b'\n(' + re.escape(b'\x1b[m\x0f\x1b[31m\x1b[1m') + b')?' +
            br'warning: (' + re.escape(b'\x1b[m\x0f\x1b[1m') +
            br')?found TODO')
        self._assert_task(['fmt-check', '--', '-V'],
                          br'\nrustfmt \S+-nightly \(\S+ 2\d\d\d-\d\d-\d\d\)')
        # TODO: Assert that the files were not modified

    # TODO: I need to decide how best to test these commands:
    # - add, rm, and update

    # TODO: The following commands need to be tested by overriding the commands
    #       to simulate --dry-run:
    # - install, install-cargo-deps, install-rustup-deps, uninstall,
    #   install-deps, install-apt-deps

    def test_kcachegrind(self):
        """just kcachegrind

        NOTE: Overrides _cargo to test what matters quickly.
        """
        callgrind_temp = self.vars['callgrind_out_file']

        for arg in ([], ['--set', 'build_flags', ' --release']):
            # TODO: Remove the target binary to verify correct paths get built
            if os.path.exists(callgrind_temp):
                os.remove(callgrind_temp)

            # The justfile echoing and the command output are both checked
            # to ensure a --release can't sneak in.
            output = self._assert_task(['kcachegrind'] + arg + [
                    '--set', 'kcachegrind', 'echo kcachegrind-foo'],
                b'\necho kcachegrind-foo \'callgrind.out.justfile\'\n'
                b'kcachegrind-foo callgrind.out.justfile\n')
            self.assertNotIn(b'--release', output)
            self.assertTrue(os.path.isfile(callgrind_temp))
            os.remove(callgrind_temp)

            output = self._assert_task(['kcachegrind'] + arg + [
                '--set', 'kcachegrind',
                'echo kcachegrind-bar', '--', '--help'], b'.*USAGE:.*')
            self.assertNotIn(b'--release', output)
            self.assertTrue(os.path.isfile(callgrind_temp))

    def test_kcov(self):
        """just kcov"""
        outdir = 'target/kcov/html'

        for arg in ([], ['--set', 'build_flags', ' --release']):
            if os.path.exists(outdir):
                shutil.rmtree(outdir)

            # The justfile echoing and the command output are both checked
            # to ensure a --release can't sneak in.
            output = self._assert_task(['kcov'] + arg, br'\ntest result:')
            self.assertNotIn(b'--release', output)
            self.assertTrue(os.path.isdir(outdir))

    def test_run(self):
        """just run"""
        self._assert_task(['run', '--', '--help'], br'\nUSAGE:')

        try:
            subprocess.check_output(['just', 'run', '/bin/sh'],
                                    stderr=subprocess.STDOUT)
        except subprocess.CalledProcessError as err:
            self.assertIn(b"panicked at 'not yet implemented'", err.output)
        else:
            self.fail("Called process should have panic'd at `unimplemented!`")

    def test_test(self):
        """just test (and the default command)"""
        for subcommand in ([], ['test']):
            self._assert_task(subcommand, br'\ntest result: ')

if __name__ == '__main__':
    print("NOTE: This test suite will currently fail unless you manually edit "
          "Cargo.toml to set a valid package name.")
    os.chdir(os.path.join(os.path.dirname(__file__), 'template'))
    unittest.main()


# vim: set sw=4 sts=4 expandtab :
