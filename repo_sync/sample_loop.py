"""
For speed comparison
"""


def py_loop(max_value: int) -> int:
    """
    python loop test
    """
    ans = 0
    for i in range(max_value):
        ans += i
    return ans
