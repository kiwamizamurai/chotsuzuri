package rest

import (
	"net/http"
	"chotsuzuri/service"
	"github.com/labstack/echo/v4"
)

type AccountHandler struct {
	service *service.AccountService
}

func NewAccountHandler(s *service.AccountService) *AccountHandler {
	return &AccountHandler{service: s}
}

func (h *AccountHandler) ListAccounts(c echo.Context) error {
	accounts, err := h.service.ListAccounts()
	if err != nil {
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}
	return c.JSON(http.StatusOK, accounts)
} 