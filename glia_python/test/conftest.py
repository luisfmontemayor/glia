import pytest
from glia_python import _global_config

@pytest.fixture(autouse=True)
def reset_global_config():
    _global_config.clear()
    yield
    _global_config.clear()
