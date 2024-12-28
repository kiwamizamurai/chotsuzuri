from dataclasses import dataclass
from typing import List, Dict

@dataclass
class ReceiptItem:
    name: str
    price: int

@dataclass
class ReceiptData:
    date: str
    amount: int
    payee: str
    items: List[ReceiptItem]

    @classmethod
    def from_dict(cls, data: Dict) -> 'ReceiptData':
        items = [ReceiptItem(**item) for item in data['items']]
        return cls(
            date=data['date'],
            amount=data['amount'],
            payee=data['payee'],
            items=items
        )