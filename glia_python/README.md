# glia_python
Lightweight Python client for the **Glia** push-architecture observability suite. Specifically designed for monitoring ephemeral, short-lived jobs with minimal overhead.

---
## Installation

Add it to your project using `uv`:

```bash
uv add glia_python
```

Or via pip:
```bash
pip install glia_python
```

## Quick Start
Glia tracks system telemetry (CPU, RAM, Wall Time) and pushes it to your central Glia backend automatically.

### Using the context manager
```python
from glia import Glia, track

with Glia.tracker(program_name="data_cleanup"):
    print("Doing heavy work...")

@track(program_name="compute_metrics")
def my_function():
    print("Computing...")
    # Raising an error here will correctly mark the job as 'Failed' in Glia
    # raise ValueError("Oops")
```


## Features
- Push-Architecture: No polling required. Data is sent directly to the Glia FastAPI backend.
- Low Footprint: Uses psutil for efficient system metric collection.
- Ephemeral Friendly: Perfectly suited for CI/CD jobs, scripts, and one-off tasks.
- Zero-Config Defaults: Automatically captures hostname, user, and script metadata.