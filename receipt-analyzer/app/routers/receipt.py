from fastapi import APIRouter, UploadFile, File
from ..services.receipt_service import analyze_receipt_with_llm, extract_text_with_ocr, analyze_text_with_llm, encode_image_to_base64
from typing import List, Optional
from pydantic import BaseModel, Field
from datetime import datetime

class AccountResponse(BaseModel):
    id: int
    code: str
    name: str

class JournalEntryResponse(BaseModel):
    id: str
    account: AccountResponse
    is_debit: bool = Field(alias="isDebit")
    amount: int

    class Config:
        allow_population_by_field_name = True
        json_encoders = {
            datetime: lambda v: v.strftime("%Y-%m-%dT%H:%M:%SZ")
        }

class JournalResponse(BaseModel):
    id: str
    journal_number: str = Field(alias="journalNumber")
    date: str
    description: str
    entries: List[JournalEntryResponse]

    class Config:
        allow_population_by_field_name = True
        json_encoders = {
            datetime: lambda v: v.strftime("%Y-%m-%dT%H:%M:%SZ")
        }

router = APIRouter(
    prefix="/api",
    tags=["receipt"]
)

@router.post("/extract-receipt-llm", response_model=JournalResponse)
async def extract_receipt_llm(file: UploadFile = File(...)):
    content = await file.read()
    image_base64 = encode_image_to_base64(content)
    receipt_data = analyze_receipt_with_llm(image_base64)
    journal = receipt_data.to_journal()
    return journal.to_dict()

@router.post("/extract-receipt-ocr")
async def extract_receipt_ocr(file: UploadFile = File(...)) -> str:
    content = await file.read()
    return extract_text_with_ocr(content)

@router.post("/extract-receipt-ocr-llm", response_model=JournalResponse)
async def extract_receipt_ocr_llm(file: UploadFile = File(...)):
    content = await file.read()
    text = extract_text_with_ocr(content)
    receipt_data = analyze_text_with_llm(text)
    journal = receipt_data.to_journal()
    return journal.to_dict()