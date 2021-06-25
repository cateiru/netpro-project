"""
@version: 1.0

Copyright (C) 2021 Netpro Project RepoSync
"""
from . import core  # type: ignore


def fop(file_path: str, dir_path: str) -> None:
    """
    file operation.
    - Create cache file.
    - File update confirmation.
    - File update.

    Args:
        file_path (str): Tartget file path.
        dir_path (str): cache directory
    """
    core.fop(file_path, dir_path)


def show(dir_path: str, use_pager: bool) -> None:
    """
    show history

    Args:
        dir_path (str): path of cache directory.
        use_pager (bool): Whether to use pager.
    """
    core.show(dir_path, use_pager)


def apply(hash_value: str, dir_path: str, file_path: str) -> None:
    """
    Apply changes from history.

    Args:
        hash_value (str): hash of history.
        dir_path (str): path of cache directory.
        file_path (str): Tartget file path.
    """
    core.apply(hash_value, dir_path, file_path)


def test_loop(max_value: int) -> int:
    """
    test loop

    Args:
        max_value (int): max loop size.

    Returns:
        int: all loop sum
    """
    return core.test_loop(max_value)
