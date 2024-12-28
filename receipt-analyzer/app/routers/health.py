from fastapi import APIRouter
from datetime import datetime
import requests
from ..models.health import HealthResponse, Component

router = APIRouter(tags=["health"])

@router.get("/health")
async def health_check() -> HealthResponse:
    health = HealthResponse(
        status="pass",
        version="1.0.0",
        service_id="receipt-analyzer-service",
        description="Receipt Analyzer Service Health Check",
        details={}
    )

    try:
        response = requests.get("http://ollama:11434/api/version")
        ollama_status = "pass"
        ollama_output = ""
        observed_value = response.json() if response.ok else None
    except Exception as e:
        ollama_status = "fail"
        ollama_output = str(e)
        observed_value = None
        health.status = "fail"

    health.details["ollama:api"] = [
        Component(
            component_type="llm",
            observed_value=observed_value,
            status=ollama_status,
            time=datetime.now(),
            output=ollama_output
        )
    ]

    return health