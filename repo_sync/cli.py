"""
Cli

Copyright (C) 2021 Netpro Project RepoSync
"""
import logging
from typing import List

import click

from .server import Server
from .client import client
from .core_op import show as core_show, apply as core_apply

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
    client(address, file_path)


@click.command()
@click.option('--port', '-p', multiple=True, prompt=False,
              help="Use port", required=True, type=int)
def server_cli(port: int):
    """
    Server

    Args:
        port (int): use port.
    """
    server = Server(port)
    server.connect()


@click.group()
def git_cli() -> None:
    """
    File update and show log.
    """


@git_cli.command()
@click.option('--pager', '-p', 'use_pager', help="use pager", default=True)
def show(use_pager: bool) -> None:
    """
    show logs.
    """
    core_show(".cache", use_pager)


@git_cli.command()
@click.option('--hash', '-h', 'hash_value', prompt=True, help="A hash of the history to apply.", required=True)
@click.option('--file', '-f', 'file_path', type=click.Path(exists=True), prompt=True,
              help="File path to update.", required=True)
def apply(hash_value: str, file_path: str) -> None:
    """
    Specify a hash to undo history changes.

    Args:
        hash (str): A hash of the history to apply.
    """
    _LOG.info("Apply hash of: %s", hash_value)
    core_apply(hash_value, '.cache', file_path)
    _LOG.info("Update file: %s", file_path)
