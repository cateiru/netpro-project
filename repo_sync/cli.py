"""
Cli

Copyright (C) 2021 Netpro Project RepoSync
"""
import logging
from typing import List
import datetime

import click

from .file_op import fop

logging.basicConfig(filename='log.txt',
                    format="%(asctime)s %(levelname)s %(message)s")
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)


@click.command()
@click.option('--address', '-a', multiple=True, prompt=False,
              help="Client address to synchronize.", required=True)
@click.option('--file', '-f', 'file_path', type=click.Path(exists=True), prompt=True,
              help="File path to synchronize.", required=True)
def sync(address: List[str], file_path: str) -> None:
    """
    RepoSync cli

    Args:
        address (List[str]): client address to synchronize.
        file_path (str): file path to synchronize.
    """
    logging.basicConfig(filename='log.txt', filemode='w')
    _LOG.info("address: %s", ", \n".join(address))
    _LOG.info("file %s", file_path)

    fop(file_path, '.cache')


@click.command()
@click.option('--repo-git', type=click.File('r'), prompt=True, help="view log", required=True)
def git_log(repo_git: str) -> None:
    """
    repo_git (str): read log.txt
    """
    with repo_git as f:
        while True:
            line = f.readline()
            if not line:
                break
            click.echo(line, nl=False)


    
    