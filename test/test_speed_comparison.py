import time
import logging

from repo_sync import rs_loop, py_loop

_LOG = logging.getLogger(__name__)


def test_comparison():
    max_value = 100000000

    start = time.time()
    ans_rs = rs_loop(max_value)
    rust_time = time.time() - start

    start = time.time()
    ans_py = py_loop(max_value)
    py_time = time.time() - start

    assert ans_rs == ans_py

    _LOG.debug("Rust time: %lf", rust_time)
    _LOG.debug("Python time: %lf", py_time)
