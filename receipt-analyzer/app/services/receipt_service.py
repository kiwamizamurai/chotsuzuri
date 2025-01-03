import base64
import requests
import json
from fastapi import HTTPException
import structlog
from PIL import Image
import io
import pytesseract
from ..models.receipt import ReceiptData

logger = structlog.get_logger()
OLLAMA_API_URL = "http://ollama:11434/api/generate"

def encode_image_to_base64(file_content: bytes) -> str:
    return base64.b64encode(file_content).decode('utf-8')

def analyze_receipt_with_llm(image_base64: str) -> ReceiptData:
    prompt = """
    You are a professional receipt analyzer. Your task is to extract information from receipts.

    STRICT RULES - VIOLATIONS ARE NOT ALLOWED:
    1. NO commas in numbers
    2. NO decimal points in prices - round to nearest integer
    3. NO currency symbols

    Required JSON format (EXACTLY as shown):
    {
        "date": "YYYY-MM-DD",
        "amount": 1234,
        "payee": "Business Name",
        "items": [
            {
                "name": "Product Name",
                "price": 1234
            }
        ]
    }

    FINAL CHECK:
    - Verify ALL numbers are integers
    """

    payload = {
        "model": "llava",
        "prompt": prompt,
        "images": [image_base64],
        "stream": False,
        "temperature": 0.1
    }

    try:
        response = requests.post(OLLAMA_API_URL, json=payload)
        result = response.json()

        if 'error' in result:
            logger.error("ollama_api_error", error=result['error'])
            raise HTTPException(
                status_code=500,
                detail=f"Ollama model error: {result['error']}"
            )

        logger.info("llm_response", response=result['response'])

        try:
            receipt_dict = json.loads(result['response'])
            return ReceiptData.from_dict(receipt_dict)
        except json.JSONDecodeError:
            raise HTTPException(
                status_code=500,
                detail="Failed to parse LLM output as JSON"
            )
        except KeyError as e:
            raise HTTPException(
                status_code=500,
                detail=f"Missing required field: {str(e)}"
            )

    except requests.exceptions.RequestException as e:
        raise HTTPException(
            status_code=500,
            detail=f"Communication error with Ollama API: {str(e)}"
        )

def extract_text_with_ocr(image_bytes: bytes) -> str:
    image = Image.open(io.BytesIO(image_bytes))
    text = pytesseract.image_to_string(image, lang='jpn')
    logger.info("ocr_result", text=text)
    return text

def analyze_text_with_llm(text: str) -> ReceiptData:
    logger.info("analyze_text_input", text=text)
    prompt = f"""
    You are a professional receipt analyzer. Analyze the following receipt text and extract information.

    Receipt Text:
    {text}

    STRICT RULES - VIOLATIONS ARE NOT ALLOWED:
    1. NO commas in numbers
    2. NO decimal points in prices - round to nearest integer
    3. NO currency symbols
    4. Keep original item names (do not translate)
    5. Keep original business name as payee (do not translate)
    6. Output ONLY valid JSON - no explanations or additional text

    Required JSON format (EXACTLY as shown):
    {{
        "date": "YYYY-MM-DD",
        "amount": 1234,
        "payee": "Original Business Name",
        "items": [
            {{
                "name": "Original Product Name",
                "price": 1234
            }}
        ]
    }}

    FINAL CHECK:
    - Verify ALL numbers are integers
    - Verify item names and payee are in original language
    - Verify date is in YYYY-MM-DD format
    - Verify output is ONLY valid JSON
    """

    payload = {
        "model": "llama2",
        "prompt": prompt,
        "stream": False,
        "temperature": 0.1
    }

    try:
        response = requests.post(OLLAMA_API_URL, json=payload)
        result = response.json()

        if 'error' in result:
            logger.error("ollama_api_error", error=result['error'])
            raise HTTPException(
                status_code=500,
                detail=f"Ollama model error: {result['error']}"
            )

        logger.info("llm_response", response=result['response'])

        try:
            receipt_dict = json.loads(result['response'])
            return ReceiptData.from_dict(receipt_dict)
        except json.JSONDecodeError:
            raise HTTPException(
                status_code=500,
                detail="Failed to parse LLM output as JSON"
            )
        except KeyError as e:
            raise HTTPException(
                status_code=500,
                detail=f"Missing required field: {str(e)}"
            )

    except requests.exceptions.RequestException as e:
        raise HTTPException(
            status_code=500,
            detail=f"Communication error with Ollama API: {str(e)}"
        )