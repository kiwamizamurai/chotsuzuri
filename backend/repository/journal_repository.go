package repository

import (
	"chotsuzuri/domain"
	"gorm.io/gorm"
	"fmt"
	"time"
)

type JournalRepository struct {
	db *gorm.DB
}

func NewJournalRepository(db *gorm.DB) *JournalRepository {
	return &JournalRepository{db: db}
}

func (r *JournalRepository) Create(journal *domain.Journal) error {
	var lastJournal domain.Journal
	r.db.Order("id desc").First(&lastJournal)
	journal.JournalNumber = fmt.Sprintf("J%d%03d", time.Now().Unix(), lastJournal.ID+1)

	return r.db.Create(journal).Error
}

func (r *JournalRepository) Update(id string, journal *domain.Journal) error {
	return r.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Where("journal_id = ?", id).Delete(&domain.JournalEntry{}).Error; err != nil {
			return err
		}

		if err := tx.Model(&domain.Journal{}).Where("id = ?", id).
			Updates(map[string]interface{}{
				"date":        journal.Date,
				"description": journal.Description,
			}).Error; err != nil {
			return err
		}

		for i := range journal.Entries {
			journal.Entries[i].JournalID = journal.ID
		}
		return tx.Create(&journal.Entries).Error
	})
}

func (r *JournalRepository) Delete(id string) error {
	return r.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Where("journal_id = ?", id).Delete(&domain.JournalEntry{}).Error; err != nil {
			return err
		}
		return tx.Delete(&domain.Journal{}, id).Error
	})
}

func (r *JournalRepository) FindByID(id string) (*domain.Journal, error) {
	var journal domain.Journal
	err := r.db.Preload("Entries").First(&journal, id).Error
	if err != nil {
		return nil, err
	}
	return &journal, nil
}

func (r *JournalRepository) ListWithFilter(page, perPage int, filter *domain.JournalFilter) ([]domain.Journal, int64, error) {
	query := r.db.Model(&domain.Journal{})

	if filter != nil {
		if filter.DateRange.From != nil {
			query = query.Where("journals.date >= ?", filter.DateRange.From)
		}
		if filter.DateRange.To != nil {
			query = query.Where("journals.date <= ?", filter.DateRange.To)
		}

		if len(filter.AccountCodes) > 0 {
			query = query.Where("journals.id IN (?)",
				r.db.Table("journal_entries").
					Select("journal_id").
					Joins("JOIN accounts ON accounts.id = journal_entries.account_id").
					Where("accounts.code IN ?", filter.AccountCodes),
			)
		}

		if filter.AmountRange.Min != nil {
			query = query.Where(
				"EXISTS (SELECT 1 FROM journal_entries WHERE journal_entries.journal_id = journals.id AND journal_entries.amount >= ?)",
				*filter.AmountRange.Min,
			)
		}
		if filter.AmountRange.Max != nil {
			query = query.Where(
				"EXISTS (SELECT 1 FROM journal_entries WHERE journal_entries.journal_id = journals.id AND journal_entries.amount <= ?)",
				*filter.AmountRange.Max,
			)
		}
	}

	var total int64
	if err := query.Count(&total).Error; err != nil {
		return nil, 0, err
	}

	var journals []domain.Journal
	err := query.Preload("Entries.Account").
		Offset((page - 1) * perPage).
		Limit(perPage).
		Find(&journals).Error

	if err != nil {
		return nil, 0, err
	}

	return journals, total, nil
}