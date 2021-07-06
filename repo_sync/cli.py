"""
Cli

Copyright (C) 2021 Netpro Project RepoSync
"""
import logging
import socket
from pathlib import Path

import click

from .client import Client
from .core_op import apply as core_apply
from .core_op import show as core_show
from .server import Server

logging.basicConfig()
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)


@click.command()
@click.option('--address', '-a', prompt=True,
              help="Client address to synchronize.", required=True)
@click.option('--file', '-f', 'file_path', type=click.Path(exists=True), prompt=True,
              help="File path to synchronize.", required=True)
@click.option('--port', '-p', prompt=True, help="Use port", required=True, type=int)
def sync_cli(address: str, file_path: str, port: int) -> None:
    """
    RepoSync cli

    Args:
        address (str): client address to synchronize.
        file_path (str): file path to synchronize.
        port (int): connect port.
    """
    _LOG.info("address: %s", address)
    _LOG.info("port: %d", port)
    with Client(port, address) as client:
        client.connect(Path(file_path), Path('.cache'))


@click.command()
@click.option('--port', '-p', prompt=True, help="Use port", required=True, type=int)
@click.option('--address', '-a', prompt=True,
              help="Self address", required=False, default=socket.gethostname())
def server_cli(port: int, address: str):
    """
    Server

    Args:
        port (int): use port.
        address (str): self address.
    """
    with Server(port, address) as server:
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

    Args:
        use_pager (bool): use pager
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
