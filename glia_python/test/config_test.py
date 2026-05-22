import os
from unittest.mock import patch
from glia_python import Glia, _global_config
from glia_python.tracker import JobTracker
from glia_python.network import push_telemetry
import pytest

@pytest.fixture(autouse=True)
def reset_global_config():
    _global_config.clear()
    yield
    _global_config.clear()

def test_global_config_init():
    Glia.init(api_url="http://init-url", app_name="init-app", app_version="1.0.0", tags={"init": "tag"})
    assert _global_config["api_url"] == "http://init-url"
    assert _global_config["app_name"] == "init-app"
    assert _global_config["app_version"] == "1.0.0"
    assert _global_config["tags"] == {"init": "tag"}

@patch.dict(os.environ, {"GLIA_APP_NAME": "env-app", "GLIA_API_URL": "http://env-url"})
def test_tracker_merges_config():
    Glia.init(app_version="2.0.0", tags={"init": "tag", "shared": "init-val"})
    
    tracker = JobTracker(context={"local": "tag", "shared": "local-val"})
    
    assert tracker._user_meta["local"] == "tag"
    assert tracker._user_meta["init"] == "tag"
    assert tracker._user_meta["shared"] == "local-val"  # Local overrides Init
    assert tracker._user_meta["app_name"] == "env-app"
    assert tracker._user_meta["app_version"] == "2.0.0"

def test_network_uses_merged_api_url():
    # If init is set, it overrides env
    with patch.dict(os.environ, {"GLIA_API_URL": "http://env-url"}):
        Glia.init(api_url="http://init-url")
        from glia_python.network import _get_api_url
        assert _get_api_url() == "http://init-url"

