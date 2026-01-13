import functools
from collections.abc import Callable
from typing import Any

from .job_tracker import JobMetrics, JobTracker

__all__: list[str] = ["JobTracker", "JobMetrics", "Glia", "track"]


class Glia:
    @staticmethod
    def tracker(program_name: str | None = None) -> JobTracker:
        """
        Context manager for tracking a block of code.

        Usage:
            with Glia.tracker(program_name="data_cleanup"):
                ...
        """
        return JobTracker(program_name=program_name)

    @staticmethod
    def track(
        program_name: str | Callable[..., Any] | None = None,
    ) -> Callable[..., Any]:
        """
        Decorator for tracking a specific function execution.
        Supports both @Glia.track() and @Glia.track usage.
        """
        if callable(program_name):
            func = program_name
            return Glia.track(program_name=None)(func)

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            @functools.wraps(func)
            def wrapper(*args: Any, **kwargs: Any) -> Any:
                func_name = getattr(func, "__name__", type(func).__name__)
                effective_name = program_name or func_name

                with JobTracker(program_name=effective_name):
                    return func(*args, **kwargs)

            return wrapper

        return decorator


track = Glia.track
