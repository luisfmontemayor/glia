from typing import final

@final
class PushResult:
    """
    Result object returned by the Rust push_telemetry function.
    Contains the raw HTTP status code and response body.
    """

    status: int
    body: str

def push_telemetry(json_payload: str, url: str, timeout: float) -> PushResult:
    """
    Pushes the JSON payload to the Glia backend.

    Args:
        json_payload: A valid JSON string representing JobMetrics.
        url: The base URL of the Glia backend (e.g., http://localhost:8000).
        timeout: Request timeout in seconds.

    Returns:
        PushResult: An object containing .status (int) and .body (str).

    Raises:
        ConnectionError: If the network is unreachable (DNS, Refused, Timeout).
    """
    ...
