#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""A utility to inject/update an HTML-format reference for a Justfile into
README.md so it's easy to keep documentation current.
"""
# TODO: RIIR

from __future__ import (absolute_import, division, print_function,
                        with_statement, unicode_literals)

__author__ = "Stephan Sokolow (deitarion/SSokolow)"
__appname__ = "HTML Justfile Reference Generator"
__version__ = "0.1"
__license__ = "Apache-2.0 OR MIT"

import logging, os, re, shutil, subprocess, tempfile
from collections import OrderedDict
from textwrap import TextWrapper

RE_STRONG = re.compile(r'\*\*([^*]*?)\*\*')
RE_BACKTICKS = re.compile(r'`([^`]*?)`')
RE_HYPERLINK = re.compile(r'\[(?P<title>[^]]+)\]\((?P<url>[^)]*)\)')
RE_TARGET_BLOCK = re.compile(
    r'(?P<tag_start><!-- BEGIN JUSTFILE TABLE: (?P<tid_start>\S+) -->\n*)'
    r'(?P<content>.*?)'
    r'(?P<tag_end>\n*<!-- END JUSTFILE TABLE: (?P<tid_end>\S+) -->)',
    re.DOTALL)
RE_VARIABLE = re.compile(r'^\s*(?P<key>\S*)\s*=\s*"(?P<value>.*?)"\s*$')

RE_GROUP = re.compile(r"^#\s*--+\s+(?P<title>.*?)\s+--+\s*$")
RE_COMMAND = re.compile(r"^@?(?P<name>\S+)\s*(?P<args>[^:]*?):[^:\n]*$")
RE_VARIABLE_RAW = re.compile(
    r"^(export\s*)?(?P<key>\S+)\s*=\s*(?P<value>.*?)\s*$")

log = logging.getLogger(__name__)
wrapper = TextWrapper(width=80, expand_tabs=False,
                      replace_whitespace=False, drop_whitespace=False,
                      break_long_words=False, break_on_hyphens=False)

class Row(list):
    """List subclass which can also have attributes as a convenience"""
    uses_variables = False

def get_evaluated_variables(include_private=False, cwd=None):
    """Call `just --evaluate` and parse it into a list of tuples"""
    results = {}
    for line in subprocess.check_output(['just', '--evaluate'],
                                        cwd=cwd).split(b'\n'):
        line = line.decode('utf8').strip()
        if not line or (line.startswith('_') and not include_private):
            continue  # Skip "private" variables

        match = RE_VARIABLE.match(line)
        if match:
            results[match.group('key')] = match.group('value')
        else:
            log.warning("Unexpected line: %r", line)

    return results

def parse_justfile(justfile, evaluated=None):
    """Parse a justfile into a grouped set of rows, but substitute `evaluated`
    if provided.
    """

    current_group = ''
    description = ''
    last_command = None

    data = {'variables': (('Variable', 'Default Value', 'Description'),
                          OrderedDict()),
            'commands': (('Command', 'Arguments', 'Description'),
                         OrderedDict())}

    # Reminder: Do *not* strip. Leading whitespace is significant.
    for line in justfile.split('\n'):
        # Let empty lines mark boundaries of doc-comments
        if not line.strip():
            description = ""
            continue

        # Persist the most recent group header
        group_match = RE_GROUP.match(line)
        if group_match:
            description = ""
            current_group = group_match.group('title').strip()
            continue

        # Accumulate potential doc comments
        if line.startswith('#'):
            description += ' ' + line.lstrip('#').strip()
            continue

        # Add variables to the current group
        var_match = RE_VARIABLE_RAW.match(line)
        if var_match:
            key, value = var_match.group('key'), var_match.group('value')

            # Skip private/internal variables
            if key.startswith('_'):
                continue

            data['variables'][1].setdefault(current_group, []).append(
                Row((key, evaluated.get(key, value), description.strip())))
            description = ''
            continue

        # Add commands to the current group
        cmd_match = RE_COMMAND.match(line)
        if cmd_match:
            name, args = cmd_match.group('name'), cmd_match.group('args')

            if not last_command:
                current_group = ''

            last_command = Row((name, args, description.strip()))
            data['commands'][1].setdefault(current_group, []).append(
                last_command)
            continue

        if last_command and line.startswith('\t') and (
                '{{' in line or '$' in line):
            last_command.uses_variables = True

    # Keep the groups in the order they were discovered, but sort the entries
    for d_type in data.values():
        for group in d_type[1].values():
            group.sort(key=lambda x: x[0])

    return data


def render_table(headers, groups):
    """Render a set of rows into a table with *exactly* the formatting
    I used to maintain by hand.
    """
    result = "<table>\n<tr>"
    for title in headers:
        result += "<th>{}</th>".format(title)
    result += "</tr>\n"
    for title, rows in groups.items():
        if title:
            result += '<tr><th colspan="{}">{}</th></tr>\n'.format(
                len(headers), RE_BACKTICKS.sub(r'<code>\1</code>', title))
        for row in rows:
            result += "<tr>\n"
            for idx, cell in enumerate(row):
                if cell.strip():
                    if cell.strip() == '+args=""':
                        cell = "args&nbsp;(optional)"
                    elif idx in (0, 1):
                        cell = '<code>{}</code>'.format(cell)
                    else:
                        cell = RE_STRONG.sub(r'<strong>\1</strong>', cell)
                        cell = RE_HYPERLINK.sub(r'<a href="\2">\1</a>', cell)
                        cell = RE_BACKTICKS.sub(r'<code>\1</code>', cell)
                        cell = '\n  '.join(
                            x.strip() for x in wrapper.wrap(cell))

                #if idx == 1 and row.uses_variables:
                #    cell += "<sub>&dagger;</sub>"
                result += "  <td>{}</td>\n".format(cell)
            result += "</tr>\n"
    result += "</table>"
    return result

def update_readme(tables):
    """Update README.md with the parsed justfile tables"""
    with open('README.md') as fobj:
        readme = fobj.read()

    def matcher(match_obj):
        """Matcher to insert/update table blocks"""
        tid_start = match_obj.group('tid_start')
        tid_end = match_obj.group('tid_end')

        assert tid_start == tid_end, "{} != {}".format(tid_start, tid_end)
        return '{}{}{}'.format(
            match_obj.group('tag_start'),
            render_table(*tables[tid_start]),
            match_obj.group('tag_end'))

    readme = RE_TARGET_BLOCK.sub(matcher, readme)

    # Atomically replace the original README.md
    tmp_dir = tempfile.mkdtemp(dir=os.getcwd())
    tmp_path = os.path.join(tmp_dir, 'README.md')
    try:
        with open(tmp_path, 'w') as fobj:
            fobj.write(readme)

        if os.name == 'nt':
            # os.rename for overwrite will fail on Windows
            os.remove('README.md')
        os.rename(tmp_path, 'README.md')
    finally:
        shutil.rmtree(tmp_dir)

def main():
    """The main entry point, compatible with setuptools entry points."""
    # If we're running on Python 2, take responsibility for preventing
    # output from causing UnicodeEncodeErrors. (Done here so it should only
    # happen when not being imported by some other program.)
    import sys
    if sys.version_info.major < 3:
        # pylint: disable=undefined-variable
        reload(sys)  # NOQA
        sys.setdefaultencoding('utf-8')  # pylint: disable=no-member

    from argparse import ArgumentParser, RawDescriptionHelpFormatter
    parser = ArgumentParser(formatter_class=RawDescriptionHelpFormatter,
            description=__doc__.replace('\r\n', '\n').split('\n--snip--\n')[0])
    parser.add_argument('--version', action='version',
            version="%%(prog)s v%s" % __version__)
    parser.add_argument('-v', '--verbose', action="count",
        default=2, help="Increase the verbosity. Use twice for extra effect.")
    parser.add_argument('-q', '--quiet', action="count",
        default=0, help="Decrease the verbosity. Use twice for extra effect.")
    # Reminder: %(default)s can be used in help strings.

    args = parser.parse_args()

    # Set up clean logging to stderr
    log_levels = [logging.CRITICAL, logging.ERROR, logging.WARNING,
                  logging.INFO, logging.DEBUG]
    args.verbose = min(args.verbose - args.quiet, len(log_levels) - 1)
    args.verbose = max(args.verbose, 0)
    logging.basicConfig(level=log_levels[args.verbose],
                        format='%(levelname)s: %(message)s')

    os.chdir(os.path.dirname(__file__))
    with open(os.path.join('template', 'justfile')) as fobj:
        justfile = fobj.read()

    tables = parse_justfile(justfile, get_evaluated_variables(cwd='template'))

    update_readme(tables)

if __name__ == '__main__':
    main()

# vim: set sw=4 sts=4 expandtab :
