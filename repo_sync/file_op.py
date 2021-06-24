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
