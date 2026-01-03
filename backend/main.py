from contextlib import asynccontextmanager
from typing import Annotated

from fastapi import Depends, FastAPI, HTTPException, status
from sqlalchemy.ext.asyncio import AsyncSession
from sqlmodel import select

from database import engine, get_session
from models import Job, JobCreate, JobRead

SessionDep = Annotated[AsyncSession, Depends(get_session)]


@asynccontextmanager
async def lifespan(app: FastAPI):
    yield
    await engine.dispose()


app = FastAPI(
    title="Glia API",
    description="Push-architecture observability for ephemeral jobs",
    version="0.1.0",
    lifespan=lifespan,
)


@app.get("/health")
async def health_check():
    return {"status": "healthy"}


@app.post("/ingest", response_model=JobRead, status_code=status.HTTP_201_CREATED)
async def ingest_telemetry(
    telemetry_in: JobCreate,
    session: SessionDep,
):
    try:
        db_telemetry = Job.model_validate(telemetry_in)
        session.add(db_telemetry)
        await session.commit()
        await session.refresh(db_telemetry)
        return db_telemetry
    except Exception as e:
        await session.rollback()
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="Failed to persist telemetry data.",
        ) from e


@app.get("/telemetry", response_model=list[JobRead])
async def list_telemetry(session: SessionDep, limit: int = 100):
    statement = select(Job).limit(limit)
    results = await session.execute(statement)
    return results.scalars().all()
