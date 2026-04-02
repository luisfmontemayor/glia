from contextlib import asynccontextmanager

from fastapi import FastAPI, HTTPException, status
from sqlalchemy.engine import Result
from sqlmodel import select, text

from backend.config import logger, settings
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


@app.get("/health/live")
async def health_check():
    return {"status": "healthy"}


@app.get("/health/ready")
async def readiness_check(session: SessionDep):
    try:
        await session.execute(text("SELECT 1"))
        return {"status": "ready"}
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Database is not reachable",
        ) from e


@app.post("/ingest", response_model=list[JobRead], status_code=status.HTTP_201_CREATED)
async def ingest_telemetry(
    telemetry_in: list[JobCreate],
    session: SessionDep,
):
    try:
        job_instances = [Job.model_validate(t) for t in telemetry_in]
        session.add_all(job_instances)
        await session.commit()
        # Refreshing all might be slow and cause issues if session state is tricky
        for job in job_instances:
            await session.refresh(job)
        return job_instances
    except Exception as e:
        await session.rollback()
        logger.error(f"Error persisting telemetry: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to persist telemetry data: {str(e)}",
        ) from e


@app.get("/telemetry", response_model=list[JobRead])
async def list_telemetry(session: SessionDep, limit: int = 100):
    statement = select(Job).order_by(Job.id.desc()).limit(limit)
    results: Result[tuple[Job]] = await session.execute(statement)
    return results.scalars().all()
