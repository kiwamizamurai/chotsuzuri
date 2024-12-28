# public.accounts

## Description

## Columns

| Name         | Type         | Default                              | Nullable | Children                                            | Parents | Comment |
| ------------ | ------------ | ------------------------------------ | -------- | --------------------------------------------------- | ------- | ------- |
| id           | integer      | nextval('accounts_id_seq'::regclass) | false    | [public.journal_entries](public.journal_entries.md) |         |         |
| code         | varchar(10)  |                                      | false    |                                                     |         |         |
| name         | varchar(100) |                                      | false    |                                                     |         |         |
| account_type | varchar(20)  |                                      | false    |                                                     |         |         |

## Constraints

| Name              | Type        | Definition       |
| ----------------- | ----------- | ---------------- |
| accounts_pkey     | PRIMARY KEY | PRIMARY KEY (id) |
| accounts_code_key | UNIQUE      | UNIQUE (code)    |

## Indexes

| Name              | Definition                                                                  |
| ----------------- | --------------------------------------------------------------------------- |
| accounts_pkey     | CREATE UNIQUE INDEX accounts_pkey ON public.accounts USING btree (id)       |
| accounts_code_key | CREATE UNIQUE INDEX accounts_code_key ON public.accounts USING btree (code) |

## Relations

```mermaid
erDiagram

"public.journal_entries" }o--|| "public.accounts" : "FOREIGN KEY (account_id) REFERENCES accounts(id)"

"public.accounts" {
  integer id
  varchar_10_ code
  varchar_100_ name
  varchar_20_ account_type
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