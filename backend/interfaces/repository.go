package interfaces

import (
	"time"
	"chotsuzuri/domain"
)

type JournalFilter struct {
	FromDate       *time.Time
	ToDate         *time.Time
	AccountCodes   []string
	DepartmentCode *string
	MinAmount      *int
	MaxAmount      *int
}

type JournalRepository interface {
	Create(journal *domain.Journal) error
	Update(journal *domain.Journal) error
	Delete(id uint) error
	FindByID(id uint) (*domain.Journal, error)
	ListWithFilter(page, pageSize int, filter *JournalFilter) ([]domain.Journal, int64, error)
} 