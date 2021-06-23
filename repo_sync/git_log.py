"""
Git_log

Copyright (C) 2021 Netpro Project RepoSync
"""

import click


@click.group()
def cli():
    """
    command grouping
    """


@cli.command()
@click.option('--log', type=click.File('r'), prompt=True, help="view log", required=True)
def git_log(log: str) -> None:
    """
    repo_git (str): read log.txt
    """
    with open(log) as file:
        while True:
            line = file.readline()
            if not line:
                break
            click.echo(line, nl=False)


@cli.command()
@click.option('--apply', type=str, prompt=True, help="show hash", required=True)
def print_hash(apply: str) -> None:
    """
    apply (str): show hash
    """
    click.echo(apply)
