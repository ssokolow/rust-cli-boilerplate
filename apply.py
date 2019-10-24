#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Just a little script to start a new project from the skeleton in
this repository."""

from __future__ import (absolute_import, division, print_function,
                        with_statement, unicode_literals)

__author__ = "Stephan Sokolow (deitarion/SSokolow)"
__appname__ = "Simple Project Template Applicator"
__version__ = "0.1"
__license__ = "Apache-2.0 OR MIT"

import json, logging, os, re, shutil, subprocess, sys, tempfile, time
from distutils.spawn import find_executable

log = logging.getLogger(__name__)

try:
    import pwd
except ImportError:
    pwd = None

# Since this script is currently for a POSIX-only project, just following
# XDG conventions for where to look for config files is sufficient for now.
# TODO: Actually use this
XDG_CONFIG_DIR = os.environ.get('XDG_CONFIG_HOME',
                                os.path.expanduser('~/.config'))

# Extensions to apply template processing to
TEMPLATABLE_EXTS = ['.rs', '.toml']

# Matcher for marking lines as to be omitted from generated project files
TEMPLATE_REMOVE_RE = re.compile(r"(#|//) TEMPLATE:REMOVE\s*$")

def ensure_terminal():
    """Re-exec self in the user's preferred terminal if stdin is not a tty."""
    if not os.isatty(sys.stdin.fileno()):
        os.execvp('./xdg-terminal', ['./xdg-terminal'] + sys.argv)

def get_author():
    """Make a best effort to retrieve the current user's name and e-mail
    and combine them into a `user <email>` string.
    """
    # TODO: Query cargo configuration too

    # Query git name and e-mail info
    gc_get = ['git', 'config', '--get']
    try:
        user = subprocess.check_output(gc_get + ['user.name']).strip()
        email = subprocess.check_output(gc_get + ['user.email']).strip()

        # TODO: Make this encoding configurable?
        user, email = user.decode('utf8'), email.decode('utf8')
    except UnicodeDecodeError:
        log.error("Could not decode name/email from git as UTF-8")
        user, email = None, None
    except (OSError, subprocess.CalledProcessError):
        user, email = None, None

    # If on a Unixy system, fall back to the "Real Name" field in the account
    if pwd and not user:
        # Query the GECOS field as a fallback
        try:
            user = pwd.getpwuid(os.getuid()).pw_gecos.split(',')[0].strip()
        except KeyError:
            pass

    # Finally, fall back to the USER and EMAIL environment variables
    if not user:
        user = os.environ.get('USER', 'unknown')
    if not email:
        email = os.environ.get('EMAIL', None)

    # And now combine whatever we found
    author = []
    if user:
        author.append(user)
    if email:
        author.append('<{}>'.format(email))
    return ' '.join(author)

def parse_ignores(path, base):
    """Load a .gitignore-style file as a list of paths.

    TODO: Support generalized globs
    """
    if not os.path.isfile(path):
        return []

    base = os.path.realpath(base)
    results = []
    with open(path) as fobj:
        for line in fobj:
            line = line.strip()

            # Skip blank lines and comments
            if not line or line.startswith('#'):
                continue

            # Force paths to be relative to the root of the repo
            line = line.lstrip(os.sep)
            if os.altsep:
                line = line.lstrip(os.altsep)

            # If the path is within the repo, add it
            line = os.path.realpath(os.path.join(base, line))
            if line.startswith(base):
                results.append(line)

    return results

def init_git_history(repo_dir):
    """Delete .git if present, re-initialize, & create a new initial commit"""
    subprocess.check_call(['git', 'init', '-q'], cwd=repo_dir)
    subprocess.check_call(['git', 'add', '.'], cwd=repo_dir)
    subprocess.check_call(['git', 'commit', '-qm',
                           'Created new project from template'], cwd=repo_dir)
    log.info("Initialized git history at %s", repo_dir)

def rmpath(path):
    """Wrapper for os.remove or shutil.rmtree as appropriate"""
    if os.path.isfile(path):
        return os.remove(path)
    elif os.path.isdir(path):
        return shutil.rmtree(path)

def template_file(path, template_vars):
    """Ultra-primitive Django/Jinja/Twig/Liquid-style template applicator"""
    def timestamp_match(match_obj):
        """Callback for timestamp pattern matches"""
        return time.strftime(match_obj.group(1))

    def match(match_obj):
        """Callback for template placeholder matches"""
        keyword = match_obj.group(1)
        date_pat = r'"now"\s*\|\s*date:\s*(?:"(.*?)"|\'(.*?)\')'
        if re.match(date_pat, keyword):
            return re.sub(date_pat, timestamp_match, keyword)

        # No fallback. We want to NOTICE if templating fails
        try:
            return template_vars[keyword]
        except KeyError:
            log.critical("No such template variable: %r\n"
                         "Valid variables are:\n\t{{ %s }}\n\t"
                         '{{ "now" | date: "<strftime string>" }}',
                         match_obj.group(1), ' }}\n\t{{ '.join(template_vars))
            sys.exit(1)

    with open(path) as fobj:
        templated = re.sub(r'{{\s*(.*?)\s*}}', match, fobj.read()).split('\n')
        prepared = []
        for line in templated:
            if not TEMPLATE_REMOVE_RE.search(line):
                prepared.append(line)

        templated = '\n'.join(prepared)
        del prepared
    with open(path, 'w') as fobj:
        fobj.write(templated)

def new_project(dest_dir):
    """Apply the template to create a new project in the given folder"""
    # Make absolute paths because we're going to chdir
    dest_dir = os.path.abspath(dest_dir)
    src_dir = os.path.abspath(os.path.dirname(__file__))

    assert not os.path.exists(dest_dir)

    cur_wd = os.getcwd()
    temp_dir = tempfile.mkdtemp(dir=os.path.dirname(dest_dir))
    temp_inner = os.path.join(temp_dir, 'repo')
    try:
        # Do a local clone of the template repo, keep only the working copy
        #
        # This requires that the template be a valid git repo, but it greatly
        # simplifies ensuring that scratch files in my local copy of the
        # boilerplate repo don't wind up in newly generated projects
        subprocess.check_call(['git', 'clone', '-qq', '--',
                               src_dir, temp_inner])
        os.rename(os.path.join(temp_inner, 'template'), dest_dir)

        # Safety guard against modifying the source dir via relative paths
        os.chdir(dest_dir)

        # Process templatable files
        project_name = os.path.basename(dest_dir)
        tmpl_vars = {
            'authors': get_author(),
            'project-name': project_name.replace('_', '-'),
            'crate_name': project_name.replace('-', '_'),
        }
        for parent, _, files in os.walk(dest_dir):
            for fname in files:
                if not os.path.splitext(fname)[1].lower() in TEMPLATABLE_EXTS:
                    continue

                template_file(os.path.join(parent, fname), tmpl_vars)

        # Assert that we're not just generating the same crate over and over
        manifest = json.loads(subprocess.check_output(
            ['cargo', 'read-manifest'], cwd=dest_dir).decode('utf8'))
        manifest_name = manifest.get('name')
        assert (tmpl_vars['project-name'] in manifest_name or
                tmpl_vars['crate_name'] in manifest_name), (
            "Generated project's Cargo.toml did not contain project name")

        log.info("Created new project at %s", dest_dir)
    finally:
        shutil.rmtree(temp_dir)
        os.chdir(cur_wd)

def assert_command_in_path(command):
    """Check for the required external commands and exit on failure"""
    if not find_executable(command):
        log.critical("Could not find %r in PATH. Exiting.", command)
        sys.exit(1)

def main():
    """The main entry point, compatible with setuptools entry points."""
    from argparse import ArgumentParser, RawDescriptionHelpFormatter
    parser = ArgumentParser(formatter_class=RawDescriptionHelpFormatter,
            description=__doc__.replace('\r\n', '\n').split('\n--snip--\n')[0])
    parser.add_argument('--version', action='version',
            version="%%(prog)s v%s" % __version__)
    parser.add_argument('-v', '--verbose', action="count",
        default=2, help="Increase the verbosity. Use twice for extra effect.")
    parser.add_argument('-q', '--quiet', action="count",
        default=0, help="Decrease the verbosity. Use twice for extra effect.")
    parser.add_argument('destdir', default=None, nargs='*',
        help="The path for the new project directory")
    # Reminder: %(default)s can be used in help strings.

    args = parser.parse_args()

    # Set up clean logging to stderr
    log_levels = [logging.CRITICAL, logging.ERROR, logging.WARNING,
                  logging.INFO, logging.DEBUG]
    args.verbose = min(args.verbose - args.quiet, len(log_levels) - 1)
    args.verbose = max(args.verbose, 0)
    logging.basicConfig(level=log_levels[args.verbose],
                        format='%(levelname)s: %(message)s')

    # TODO: Rework things so ensure_terminal gets brought up first but only
    # if it is necessary
    assert_command_in_path('git')
    assert_command_in_path('cargo')

    while not args.destdir:
        ensure_terminal()
        destdir = input("Path for new project: ")
        if destdir:
            args.destdir.append(destdir)

    # TODO: If this is a first run and just isn't installed, offer to
    #       `cargo install` it.

    # TODO: Support the project name being relative to a config-file-specified
    #       parent directory.

    for path in args.destdir:
        if not os.path.exists(path):
            new_project(path)
            init_git_history(path)

        # TODO: Modulo a config file, ensure that ~/.cargo/bin is in the PATH
        #       and then open the preferred editing environment.

if __name__ == '__main__':
    main()

# vim: set sw=4 sts=4 expandtab :
