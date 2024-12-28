package rest

import (
    "net/http"
    "github.com/labstack/echo/v4"
    "gorm.io/gorm"
    "time"
)

type HealthHandler struct {
    db *gorm.DB
}

func NewHealthHandler(db *gorm.DB) *HealthHandler {
    return &HealthHandler{db: db}
}

type HealthResponse struct {
    Status      string                 `json:"status"`
    Version     string                 `json:"version"`
    ServiceID   string                 `json:"serviceId"`
    Description string                 `json:"description"`
    Details     map[string][]Component `json:"details"`
}

type Component struct {
    ComponentID   string                `json:"componentId,omitempty"`
    ComponentType string                `json:"componentType"`
    ObservedValue interface{}           `json:"observedValue"`
    Status        string                `json:"status"`
    Time          time.Time             `json:"time"`
    Output        string                `json:"output,omitempty"`
}

func (h *HealthHandler) Check(c echo.Context) error {
    health := HealthResponse{
        Status:      "pass",
        Version:     "1.0.0",
        ServiceID:   "accounting-service",
        Description: "Accounting Service Health Check",
        Details:     make(map[string][]Component),
    }

    sqlDB, err := h.db.DB()
    if err != nil {
        health.Status = "fail"
    } else {
        err = sqlDB.Ping()
        dbStatus := "pass"
        dbOutput := ""
        if err != nil {
            dbStatus = "fail"
            dbOutput = err.Error()
        }

        stats := sqlDB.Stats()
        health.Details["postgresql:connections"] = []Component{
            {
                ComponentType: "database",
                ObservedValue: stats.InUse,
                Status:       dbStatus,
                Time:        time.Now(),
                Output:      dbOutput,
            },
        }
    }

    statusCode := http.StatusOK
    if health.Status == "fail" {
        statusCode = http.StatusServiceUnavailable
    }

    return c.JSON(statusCode, health)
}