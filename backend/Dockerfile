# syntax=docker/dockerfile:1

FROM golang:1.22.5 AS base
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download

FROM base AS development
RUN go install github.com/air-verse/air@v1.52.3
COPY . .
RUN go install ./...
CMD ["air", "-c", ".air.toml"]

FROM base AS production
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -o /app-server
CMD ["/app-server"]