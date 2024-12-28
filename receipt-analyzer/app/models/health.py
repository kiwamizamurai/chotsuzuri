from pydantic import BaseModel
from typing import Dict, List, Optional, Any
from datetime import datetime

class Component(BaseModel):
    component_type: str
    observed_value: Any
    status: str
    time: datetime
    output: Optional[str] = None

class HealthResponse(BaseModel):
    status: str
    version: str
    service_id: str
    description: str
    details: Dict[str, List[Component]]