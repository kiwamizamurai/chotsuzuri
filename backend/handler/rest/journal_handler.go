package rest

import (
	"net/http"
	"time"
	"chotsuzuri/domain"
	"chotsuzuri/service"
	"github.com/labstack/echo/v4"
)

type JournalHandler struct {
	service *service.JournalService
}

func NewJournalHandler(s *service.JournalService) *JournalHandler {
	return &JournalHandler{service: s}
}

type CreateJournalRequest struct {
	Date        string                    `json:"date"`
	Description string                    `json:"description"`
	Entries     []CreateJournalEntryRequest `json:"entries"`
}

type CreateJournalEntryRequest struct {
	AccountID uint  `json:"account_id"`
	IsDebit   bool  `json:"is_debit"`
	Amount    int   `json:"amount"`
}

func (h *JournalHandler) CreateJournal(c echo.Context) error {
	var req CreateJournalRequest
	if err := c.Bind(&req); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}

	date, err := time.Parse(time.RFC3339, req.Date)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, "invalid date format")
	}

	journal := &domain.Journal{
		Date:        date,
		Description: req.Description,
		Entries:     make([]domain.JournalEntry, len(req.Entries)),
	}

	for i, entry := range req.Entries {
		journal.Entries[i] = domain.JournalEntry{
			AccountID: entry.AccountID,
			IsDebit:   entry.IsDebit,
			Amount:    entry.Amount,
		}
	}

	if err := h.service.CreateJournal(journal); err != nil {
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	return c.JSON(http.StatusCreated, journal)
}

func (h *JournalHandler) UpdateJournal(c echo.Context) error {
	id := c.Param("id")
	var req CreateJournalRequest
	if err := c.Bind(&req); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}

	date, err := time.Parse(time.RFC3339, req.Date)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, "invalid date format")
	}

	journal := &domain.Journal{
		Date:        date,
		Description: req.Description,
		Entries:     make([]domain.JournalEntry, len(req.Entries)),
	}

	for i, entry := range req.Entries {
		journal.Entries[i] = domain.JournalEntry{
			AccountID: entry.AccountID,
			IsDebit:   entry.IsDebit,
			Amount:    entry.Amount,
		}
	}

	if err := h.service.UpdateJournal(id, journal); err != nil {
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	return c.JSON(http.StatusOK, journal)
}

func (h *JournalHandler) DeleteJournal(c echo.Context) error {
	id := c.Param("id")
	if err := h.service.DeleteJournal(id); err != nil {
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}
	return c.NoContent(http.StatusNoContent)
}