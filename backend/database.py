import os
from collections.abc import AsyncGenerator

from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker, create_async_engine

from .config import settings

dev_mode: str | None = os.environ.get("GLIA_DEV_MODE")

engine = create_async_engine(
    settings.DATABASE_URL,
    echo=dev_mode is True,
    future=True,
)

AsyncSessionLocal = async_sessionmaker(
    bind=engine, class_=AsyncSession, expire_on_commit=False, autoflush=False
)


async def get_session() -> AsyncGenerator[AsyncSession, None]:
    """
    Yields a database session that automatically closes
    after the request/block is finished.
    """
    async with AsyncSessionLocal() as session:
        try:
            yield session
        finally:
            await session.close()
