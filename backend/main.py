from contextlib import asynccontextmanager

from fastapi import FastAPI, HTTPException, status
from sqlmodel import select

from backend.config import settings
from backend.database import SessionDep, engine
from backend.models import Job, JobCreate, JobRead


@asynccontextmanager
async def lifespan(app: FastAPI):
    yield
    await engine.dispose()


app = FastAPI(
    title=settings.PROJECT_TITLE,
    description=settings.PROJECT_TITLE,
    version=settings.VERSION,
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
        job_instance: Job = Job.model_validate(telemetry_in)
        session.add(instance=job_instance)
        await session.commit()
        await session.refresh(instance=job_instance)
        return job_instance
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
