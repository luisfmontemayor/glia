# glia_python/src/glia_python/__init__.py

import functools
from collections.abc import Callable
from typing import Any

from .job_tracker import JobMetrics, JobTracker

__all__: list[str] = ["JobTracker", "JobMetrics", "Glia", "track"]


class Glia:
    @staticmethod
    def tracker(name: str | None = None) -> JobTracker:
        """
        Context manager for tracking a block of code.
        The 'name' will be appended to the script name (e.g. script.py:name).
        """
        return JobTracker(block_name=name)

    @staticmethod
    def track(
        name: str | Callable[..., Any] | None = None,
    ) -> Callable[..., Any]:
        """
        Decorator. Uses the function name as the block name unless 'name' is provided.
        """
        if callable(name):
            func = name
            return Glia.track(name=None)(func)

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            @functools.wraps(func)
            def wrapper(*args: Any, **kwargs: Any) -> Any:
                func_name = getattr(func, "__name__", type(func).__name__)
                effective_name = name or func_name

                with JobTracker(block_name=effective_name):
                    return func(*args, **kwargs)

            return wrapper

        return decorator


track = Glia.track
