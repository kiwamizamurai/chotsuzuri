import pytest
from fastapi.testclient import TestClient
from app.main import app
import os

client = TestClient(app)

def test_extract_receipt_ocr():
    current_dir = os.path.dirname(os.path.abspath(__file__))
    image_path = os.path.join(current_dir, 'samples', 'sample_receipt.png')

    assert os.path.exists(image_path), "Test image file not found"

    with open(image_path, 'rb') as image:
        files = {'file': ('sample_receipt.png', image, 'image/png')}
        response = client.post("/api/extract-receipt-ocr", files=files)

    assert response.status_code == 200

    text = response.text.strip('"')

    expected_contents = [
        "領収書",
        "2024年3月20日",
        "株式会社ボッタクリサセル様",
        "3,300",
        "内訳",
        "文房具",
        "2,200",
        "コピー用紙",
        "1,100",
        "株式会社サンプル商店",
        "東京都千代田区1-2-3"
    ]

    for content in expected_contents:
        assert content in text, f"'{content}' not found in OCR results"