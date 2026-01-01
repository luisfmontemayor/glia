# Glia Database & API Implementation Roadmap

## Phase 1: Infrastructure & The Container (Docker)
*Goal: Get a Postgres instance running in an isolated environment without polluting the host OS.*
- [x] Install Docker and verify installation.
- [x] Create a `compose.yaml` file in the root directory to define the Postgres service.
  - [x] Pick a docker image
- [x] Configure environment variables (User, Password, DB Name) securely using a `.env` file.
- [ ] Launch the database and connect to it using the CLI to verify it is accepting connections.

## Phase 2: The Data Layer (ORM & Models)
*Goal: Define the database structure using Python code (SQLModel) rather than writing raw SQL.*
- [ ] Add dependencies via `uv`: `sqlmodel`, `asyncpg` (async driver), and `pydantic-settings`.
- [ ] Create `backend/database.py`: Set up the async engine and session functionality.
- [ ] Design the Schema: Create `backend/models.py` translating Glia's requirements (Usage, Resources, Data Footprint, JSONB) into SQLModel classes.
- [ ] Understand Primary Keys, Foreign Keys, and Indexes (crucial for query performance).

## Phase 3: Schema Management (Migrations)
*Goal: Handle database changes over time (version control for your DB).*
- [ ] Add dependency: `alembic`.
- [ ] Initialize Alembic in the project.
- [ ] Configure `alembic.ini` and `env.py` to work with the SQLModel async engine.
- [ ] Generate the first migration script (revision) to create the initial tables.
- [ ] Apply the migration to the running Docker Postgres instance.

## Phase 4: The API Layer (FastAPI Integration)
*Goal: Expose the database to the outside world via HTTP endpoints.*
- [ ] Add dependency: `fastapi` and `uvicorn`.
- [ ] Create `backend/main.py` and set up the base app.
- [ ] Implement Dependency Injection: Create a `get_session` dependency to manage DB transactions per request.
- [ ] Create the Ingestion Endpoint: A `POST` route that accepts the telemetry payload and writes it to the DB.
- [ ] Implement Error Handling: Ensure the API degrades gracefully if the DB is under load.

## Phase 5: Verification & robustness
*Goal: Ensure the system handles the "ephemeral" nature of your clients.*
- [ ] Write a simple test script (using `httpx`) to simulate a client pushing data.
- [ ] Inspect the data in the DB GUI to verify JSONB fields are storing correctly.
- [ ] (Optional) Add a "Keep Alive" or connection pooling check to ensure rapid CLI scripts don't exhaust DB connections.