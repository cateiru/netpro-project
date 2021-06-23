"""
Git_log

Copyright (C) 2021 Netpro Project RepoSync
"""

import click


@click.command()
@click.option('--repo-git', type=click.File('r'), prompt=True, help="view log", required=True)
def git_log(repo_git: str) -> None:
    """
    repo_git (str): read log.txt
    """
    with repo_git as file:
        while True:
            line = file.readline()
            if not line:
                break
            click.echo(line, nl=False)
