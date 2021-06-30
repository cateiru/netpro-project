"""
Cli

Copyright (C) 2021 Netpro Project RepoSync
"""
import logging
from typing import List

import click
import socket

from .core_op import fop
from .server import server
from .client import client
logging.basicConfig()
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)


@click.command()
@click.option('--address', '-a', multiple=True, prompt=False,
              help="Client address to synchronize.", required=True)
@click.option('--file', '-f', 'file_path', type=click.Path(exists=True), prompt=True,
              help="File path to synchronize.", required=True)
def sync_cli(address: List[str], file_path: str) -> None:
    """
    RepoSync cli

    Args:
        address (List[str]): client address to synchronize.
        file_path (str): file path to synchronize.
    """
    _LOG.info("address: %s", ", \n".join(address))
    _LOG.info("file %s", file_path)

    fop(file_path, '.cache')

    server(address, file_path)
    client(address, file_path)

@click.group()
def git_cli() -> None:
    """
    File update and show log.
    """


@git_cli.command()
def show() -> None:
    """
    show logs.
    """


@git_cli.command()
@click.option('--hash', '-h', 'hash_value', prompt=True, help="A hash of the history to apply.", required=True)
def applay(hash_value: str) -> None:
    """
    Specify a hash to undo history changes.

    Args:
        hash (str): A hash of the history to apply.
    """
    _LOG.info("Hash: %s", hash_value)
