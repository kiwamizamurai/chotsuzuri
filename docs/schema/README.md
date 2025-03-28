# accounting

## Tables

| Name                                                | Columns | Comment | Type       |
| --------------------------------------------------- | ------- | ------- | ---------- |
| [public.accounts](public.accounts.md)               | 4       |         | BASE TABLE |
| [public.journals](public.journals.md)               | 6       |         | BASE TABLE |
| [public.journal_entries](public.journal_entries.md) | 5       |         | BASE TABLE |

## Relations

```mermaid
erDiagram

"public.journal_entries" }o--|| "public.accounts" : "FOREIGN KEY (account_id) REFERENCES accounts(id)"
"public.journal_entries" }o--|| "public.journals" : "FOREIGN KEY (journal_id) REFERENCES journals(id) ON DELETE CASCADE"

"public.accounts" {
  integer id
  varchar_10_ code
  varchar_100_ name
  varchar_20_ account_type
}
"public.journals" {
  integer id
  varchar_20_ journal_number
  timestamp_without_time_zone date
  text description
  timestamp_without_time_zone created_at
  timestamp_without_time_zone updated_at
}
"public.journal_entries" {
  integer id
  integer journal_id FK
  integer account_id FK
  boolean is_debit
  integer amount
}
```

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
