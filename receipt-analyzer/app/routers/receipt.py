from fastapi import APIRouter, UploadFile, File
from ..services.receipt_service import analyze_receipt_with_llm, extract_text_with_ocr, analyze_text_with_llm, encode_image_to_base64
from ..models.receipt import ReceiptData

router = APIRouter(
    prefix="/api",
    tags=["receipt"]
)

@router.post("/extract-receipt-llm")
async def extract_receipt_llm(file: UploadFile = File(...)) -> ReceiptData:
    content = await file.read()
    image_base64 = encode_image_to_base64(content)
    return analyze_receipt_with_llm(image_base64)

@router.post("/extract-receipt-ocr")
async def extract_receipt_ocr(file: UploadFile = File(...)) -> str:
    content = await file.read()
    return extract_text_with_ocr(content)

@router.post("/extract-receipt-ocr-llm")
async def extract_receipt_ocr_llm(file: UploadFile = File(...)) -> ReceiptData:
    content = await file.read()
    text = extract_text_with_ocr(content)
    return analyze_text_with_llm(text)