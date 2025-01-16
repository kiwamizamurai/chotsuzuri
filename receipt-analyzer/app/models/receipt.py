from dataclasses import dataclass
from typing import List, Dict
from datetime import datetime


@dataclass
class JournalEntry:
    is_debit: bool
    amount: int

@dataclass
class Journal:
    date: datetime
    description: str
    entries: List[JournalEntry]

    def to_dict(self) -> Dict:
        return {
            "id": "RECEIPT",
            "journalNumber": "RECEIPT",
            "date": self.date.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "description": self.description,
            "entries": [
                {
                    "id": "ENTRY",
                    "account": {
                        "id": 1,
                        "code": "EXPENSE" if entry.is_debit else "CASH",
                        "name": "経費" if entry.is_debit else "現金"
                    },
                    "isDebit": entry.is_debit,
                    "amount": entry.amount
                }
                for entry in self.entries
            ]
        }

@dataclass
class ReceiptItem:
    name: str
    price: int

@dataclass
class ReceiptData:
    date: datetime
    amount: int
    payee: str
    items: List[ReceiptItem]

    @property
    def total_amount(self) -> int:
        return sum(item.price for item in self.items)

    @classmethod
    def from_dict(cls, data: Dict) -> 'ReceiptData':
        items = [ReceiptItem(**item) for item in data['items']]
        return cls(
            date=datetime.fromisoformat(data['date']),
            amount=data['amount'],
            payee=data['payee'],
            items=items
        )

    def to_journal(self) -> Journal:
        total = self.total_amount
        return Journal(
            date=self.date,
            description=self.payee,
            entries=[
                JournalEntry(
                    is_debit=True,
                    amount=total
                ),
                JournalEntry(
                    is_debit=False,
                    amount=total
                )
            ]
        )