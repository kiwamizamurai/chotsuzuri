package main

import (
    "fmt"
    "log"
    "os"

    "chotsuzuri/graph/generated"
    "chotsuzuri/graph/resolver"
    "chotsuzuri/handler/rest"
    "chotsuzuri/repository"
    "chotsuzuri/service"

    "github.com/99designs/gqlgen/graphql/handler"
    "github.com/99designs/gqlgen/graphql/playground"
    "github.com/labstack/echo/v4"
    "github.com/labstack/echo/v4/middleware"
    "gorm.io/driver/postgres"
    "gorm.io/gorm"
)

func initDB() *gorm.DB {
    host := os.Getenv("DB_HOST")
    if host == "" {
        host = "localhost"
    }
    user := os.Getenv("DB_USER")
    if user == "" {
        user = "accounting"
    }
    password := os.Getenv("DB_PASSWORD")
    if password == "" {
        password = "accounting"
    }
    dbname := os.Getenv("DB_NAME")
    if dbname == "" {
        dbname = "accounting"
    }
    port := os.Getenv("DB_PORT")
    if port == "" {
        port = "5432"
    }

    dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s sslmode=disable TimeZone=Asia/Tokyo",
        host, user, password, dbname, port)

    db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
    if err != nil {
        log.Fatal("failed to connect database", err)
    }

    return db
}

func main() {
    db := initDB()

    journalRepo := repository.NewJournalRepository(db)
    accountRepo := repository.NewAccountRepository(db)

    journalService := service.NewJournalService(journalRepo, accountRepo)
    accountService := service.NewAccountService(accountRepo)

    e := echo.New()

    e.Use(middleware.Logger())
    e.Use(middleware.Recover())
    e.Use(middleware.CORS())

    journalHandler := rest.NewJournalHandler(journalService)
    accountHandler := rest.NewAccountHandler(accountService)

    api := e.Group("/api")
    {
        journals := api.Group("/journals")
        {
            journals.POST("", journalHandler.CreateJournal)
            journals.PUT("/:id", journalHandler.UpdateJournal)
            journals.DELETE("/:id", journalHandler.DeleteJournal)
        }

        accounts := api.Group("/accounts")
        {
            accounts.GET("", accountHandler.ListAccounts)
        }
    }

    healthHandler := rest.NewHealthHandler(db)
    e.GET("/health", healthHandler.Check)

    resolvers := resolver.NewResolver(journalService, accountService)
    srv := handler.NewDefaultServer(
        generated.NewExecutableSchema(
            generated.Config{Resolvers: resolvers},
        ),
    )

    e.POST("/query", func(c echo.Context) error {
        srv.ServeHTTP(c.Response(), c.Request())
        return nil
    })

    playground := playground.Handler("GraphQL Playground", "/query")
    e.GET("/playground", func(c echo.Context) error {
        playground.ServeHTTP(c.Response(), c.Request())
        return nil
    })

    e.Logger.Fatal(e.Start(":8080"))
}