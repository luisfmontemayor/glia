# Lint Fix Summary

This document summarizes the linting fixes applied to the Glia project to improve code quality, consistency, and idiomatic correctness.

## Python (Ruff)

### 1. Unused Imports (F401)
*   **Fix:** Removed unused imports across several files, primarily in database migrations and benchmarks.
*   **Reason:** Reduces overhead, keeps code clean, and prevents confusion about dependencies.
*   **Example:**
    ```python
    # Before
    import os
    import json
    from datetime import datetime
    
    # After (if os was unused)
    import json
    from datetime import datetime
    ```

### 2. Import Sorting (I001)
*   **Fix:** Automatically reordered imports using `isort` rules.
*   **Reason:** Consistent import structure makes it easier to scan files and avoids merge conflicts.
*   **Example:**
    ```python
    # Before
    import core
    import os
    from common.logs import setup_logger
    
    # After
    import os
    import core
    from common.logs import setup_logger
    ```

### 3. Modern Type Annotations (UP007)
*   **Fix:** Replaced `Union[A, B]` with the modern `A | B` syntax (Python 3.10+).
*   **Reason:** More concise and readable syntax.
*   **Example:**
    ```python
    # Before
    def process(data: Union[str, int, None]): ...
    
    # After
    def process(data: str | int | None): ...
    ```

### 4. Collection ABCs (UP035)
*   **Fix:** Replaced imports from `typing` with `collections.abc` where appropriate.
*   **Reason:** `typing` equivalents for common collections are deprecated in favor of `collections.abc` in newer Python versions.
*   **Example:**
    ```python
    # Before
    from typing import Sequence
    
    # After
    from collections.abc import Sequence
    ```

## R (lintr)

### 1. Disallowing Explicit `return()`
*   **Fix:** Removed `return()` calls and configured `lintr` to enforce implicit returns.
*   **Reason:** Explicit `return()` at the end of a function is considered non-idiomatic in R and discouraged by the Tidyverse style guide.
*   **Example:**
    ```r
    # Before
    my_fun <- function(x) {
      return(x + 1)
    }
    
    # After
    my_fun <- function(x) {
      x + 1
    }
    ```

### 2. Configuration (`.lintr`)
*   **Action:** Created `.lintr` to centralize R linting rules. Temporarily silenced noisy style rules (line length, trailing whitespace) to focus on functional/architectural consistency.
