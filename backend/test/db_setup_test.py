import sys

from glia_common import logs, system

logger = logs.setup_logger("glia_db_checker")


def run_check(command: list[str], description: str) -> str | None:
    logger.info(f"Starting check: {description}...")

    output: str | None = system.run_command(command)

    if output:
        logger.info(f"Result: {description} -> OK")
        return output
    else:
        logger.error(f"Result: {description} -> FAILED")
        logger.debug(f"Command failed: {' '.join(command)}")
        return None


def check_status() -> bool:
    cmd: list[str] = ["docker", "compose", "ps", "--format", "{{.State}}", "db"]
    output: str | None = run_check(cmd, "Checking Docker Container Status")

    if output and "running" in output.lower():
        return True
    logger.warning("Container is not running.")
    return False


def check_logs() -> bool:
    cmd: list[str] = ["docker", "compose", "logs", "db"]
    output: str | None = run_check(cmd, "Scanning Logs for Readiness")

    if output and "database system is ready to accept connections" in output:
        return True
    logger.warning("Database hasn't signaled readiness yet.")
    return False


def check_connection(user: str, db: str) -> bool:
    cmd: list[str] = [
        "docker",
        "compose",
        "exec",
        "db",
        "psql",
        "-U",
        user,
        "-d",
        db,
        "-c",
        "SELECT 1 as connected;",
    ]
    output: str | None = run_check(
        cmd, f"Verifying Connection (User: {user}, DB: {db})"
    )

    if output and "connected" in output:
        return True
    return False


def main() -> None:
    logger.info("Starting Glia Database Verification")

    config: dict[str, str] = system.parse_env_file(".env")
    db_user: str | None = config.get("POSTGRES_USER")
    db_name: str | None = config.get("POSTGRES_DB")

    if not db_user or not db_name:
        logger.critical("POSTGRES_USER or POSTGRES_DB missing from .env")
        sys.exit(1)

    if not check_status():
        sys.exit(1)

    if not check_logs():
        logger.info("(Wait a few seconds and try again if it just started)")
        sys.exit(1)

    if not check_connection(db_user, db_name):
        sys.exit(1)

    logger.info("Success: Glia Database is healthy and accessible.")


if __name__ == "__main__":
    main()
