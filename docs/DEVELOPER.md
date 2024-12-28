
## Requirements

- mise
- docker
- docker-compose

## Architecture

### Frontend
- Built with Rust + Yew framework for WebAssembly
- GraphQL client for data fetching
- Component-based architecture
- Responsive design with CSS Grid

### Backend
- Go with Echo framework
- GraphQL API using gqlgen
- PostgreSQL database with GORM
- RESTful endpoints for basic operations

### Architecture Diagram

```mermaid
graph TB
    subgraph Frontend[Frontend - Rust/Yew]
        subgraph TEA[The Elm Architecture]
            Model[Model/State]
            Update[Update Logic]
            View[View Components]
        end
        APIClient[API Client]
    end

    subgraph Backend[Backend - Go]
        subgraph API[API Layer]
            REST[REST Handlers]
            GQL[GraphQL Resolvers]
        end

        subgraph Domain[Domain Layer]
            Service[Business Logic]
            Entities[Domain Entities]
        end

        subgraph Data[Data Layer]
            Repo[Repository Interface]
            RepoImpl[Repository Implementation]
        end
    end

    subgraph Database[PostgreSQL]
        Journals[(Journals)]
        Accounts[(Accounts)]
        Entries[(Journal Entries)]
    end

    %% Frontend Flow
    View --> Update
    Update --> Model
    Model --> View
    Update --> APIClient

    %% API Communication
    APIClient --> API

    %% Backend Flow
    REST --> Service
    GQL --> Service
    Service --> Entities
    Service --> Repo
    Repo --> RepoImpl
    RepoImpl --> Database

    %% Database Relations
    Journals --> Entries
    Accounts --> Entries
```

### Data Flows

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant B as Backend
    participant DB as Database

    %% Create Flow
    Note over U,DB: Create Journal Entry
    U->>F: Create Journal Entry
    F->>B: POST /api/journals
    B->>B: Validate Entry
    B->>DB: Begin Transaction
    DB-->>B: OK
    B->>DB: Insert Journal
    B->>DB: Insert Entries
    DB-->>B: Success
    B->>DB: Commit
    DB-->>B: OK
    B-->>F: Journal Created
    F-->>U: Show Success

    %% Read Flow
    Note over U,DB: Read Journal Entries
    U->>F: Request Journal List
    F->>B: GraphQL Query
    B->>DB: Select with Filters
    DB-->>B: Return Records
    B-->>F: Journal List
    F-->>U: Display Journals

    %% Update Flow
    Note over U,DB: Update Journal Entry
    U->>F: Modify Journal
    F->>B: PUT /api/journals/{id}
    B->>B: Validate Changes
    B->>DB: Begin Transaction
    DB-->>B: OK
    B->>DB: Delete Old Entries
    B->>DB: Update Journal
    B->>DB: Insert New Entries
    DB-->>B: Success
    B->>DB: Commit
    DB-->>B: OK
    B-->>F: Update Confirmed
    F-->>U: Show Success

    %% Delete Flow
    Note over U,DB: Delete Journal Entry
    U->>F: Delete Journal
    F->>B: DELETE /api/journals/{id}
    B->>DB: Begin Transaction
    DB-->>B: OK
    B->>DB: Delete Entries
    B->>DB: Delete Journal
    DB-->>B: Success
    B->>DB: Commit
    DB-->>B: OK
    B-->>F: Deletion Confirmed
    F-->>U: Remove from UI
```

## Development Setup

```shell
❯ mise dup
```


## Tips


```shell
❯ curl -L -X POST \
  http://localhost:8888/api/extract-receipt-ocr-llm \
  -H "Content-Type: multipart/form-data" \
  -F "file=@receipt-analyzer/tests/samples/sample_receipt.png"
{"date":"2024-03-20","amount":1234,"payee":"株式会社ボッタクリサセル様","items":[{"name":"文房具","price":2200},{"name":"コピー用紙","price":1100}]}
```


```shell
curl -X POST http://localhost:8080/api/journals \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2024-03-25T00:00:00Z",
    "description": "備品購入",
    "entries": [
      {
        "account_id": 3,
        "is_debit": true,
        "amount": 10000
      },
      {
        "account_id": 1,
        "is_debit": false,
        "amount": 10000
      }
    ]
  }'
```


```shell
curl -X PUT http://localhost:8080/api/journals/1 \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2024-03-25T00:00:00Z",
    "description": "備品購入（更新）",
    "entries": [
      {
        "account_id": 1,
        "is_debit": true,
        "amount": 15000
      },
      {
        "account_id": 2,
        "is_debit": false,
        "amount": 15000
      }
    ]
  }'
```


```shell
curl -X DELETE http://localhost:8080/api/journals/1
```

```graphql
query ListJournals {
  journals(
    pagination: { page: 1, perPage: 10 }
    filter: {
      dateRange: { from: "2024-01-01T00:00:00Z", to: "2024-12-31T23:59:59Z" }
      # accountCodes: ["1001", "1002"]
      # amountRange: { min: 1000, max: 1000000 }
    }
  ) {
    items {
      id
      journalNumber
      date
      description
      entries {
        id
        account {
          id
          code
          name
          accountType
        }
        isDebit
        amount
      }
      createdAt
      updatedAt
    }
    pageInfo {
      hasNextPage
      hasPrevPage
      totalPages
      currentPage
    }
  }
}
```
