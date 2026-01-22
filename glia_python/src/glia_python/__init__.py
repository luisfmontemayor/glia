import functools
from collections.abc import Callable
from typing import Any

from glia_python.JobMetrics import JobMetrics
from glia_python.tracker import JobTracker

__all__: list[str] = ["JobTracker", "JobMetrics", "Glia", "track"]


class Glia:
    @staticmethod
    def tracker(
        program_name: str | None = None, context: dict[str, Any] | None = None
    ) -> JobTracker:
        """
        Context manager for tracking a block of code.
        The 'program_name' will be appended to the script name (e.g. script.py:program_name).
        """
        return JobTracker(program_name=program_name, context=context)

    @staticmethod
    def track(
        program_name: str | Callable[..., Any] | None = None,
        context: dict[str, Any] | None = None,
    ) -> Callable[..., Any]:
        """
        Decorator. Uses the function name as the block name unless 'program_name' is provided.
        """
        # Handle case where used as @Glia.track without arguments
        if callable(program_name):
            func = program_name
            return Glia.track(program_name=None, context=context)(func)

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            @functools.wraps(func)
            def wrapper(*args: Any, **kwargs: Any) -> Any:
                func_name = getattr(func, "__name__", type(func).__name__)

                effective_name = program_name or func_name

                with JobTracker(program_name=effective_name, context=context):
                    return func(*args, **kwargs)

            return wrapper

        return decorator


track = Glia.track
