from fastapi import FastAPI
import structlog
import logging
from .routers import receipt, health

structlog.configure(
    processors=[
        structlog.processors.JSONRenderer()
    ],
    logger_factory=structlog.PrintLoggerFactory(),
    wrapper_class=structlog.make_filtering_bound_logger(logging.INFO),
)

app = FastAPI()

app.include_router(receipt.router)
app.include_router(health.router)