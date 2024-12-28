package service

import (
	"errors"
	"fmt"
	"chotsuzuri/domain"
	"chotsuzuri/repository"
)

type JournalService struct {
	repo *repository.JournalRepository
	accountRepo *repository.AccountRepository
}

func NewJournalService(repo *repository.JournalRepository, accountRepo *repository.AccountRepository) *JournalService {
	return &JournalService{
		repo: repo,
		accountRepo: accountRepo,
	}
}

func (s *JournalService) CreateJournal(journal *domain.Journal) error {
	if err := s.validateEntries(journal.Entries); err != nil {
		return err
	}

	if !s.validateBalances(journal.Entries) {
		return errors.New("debit and credit amounts must be equal")
	}
	return s.repo.Create(journal)
}

func (s *JournalService) UpdateJournal(id string, journal *domain.Journal) error {
	existing, err := s.repo.FindByID(id)
	if err != nil {
		return err
	}
	if existing == nil {
		return errors.New("journal not found")
	}

	if !s.validateBalances(journal.Entries) {
		return errors.New("debit and credit amounts must be equal")
	}

	return s.repo.Update(id, journal)
}

func (s *JournalService) DeleteJournal(id string) error {
	existing, err := s.repo.FindByID(id)
	if err != nil {
		return err
	}
	if existing == nil {
		return errors.New("journal not found")
	}

	return s.repo.Delete(id)
}

func (s *JournalService) ListJournals(page, perPage int, filter *domain.JournalFilter) ([]domain.Journal, int64, error) {
	return s.repo.ListWithFilter(page, perPage, filter)
}

func (s *JournalService) validateBalances(entries []domain.JournalEntry) bool {
	var debitSum, creditSum int
	for _, entry := range entries {
		if entry.IsDebit {
			debitSum += entry.Amount
		} else {
			creditSum += entry.Amount
		}
	}
	return debitSum == creditSum
}

func (s *JournalService) validateEntries(entries []domain.JournalEntry) error {
	for _, entry := range entries {
		account, err := s.accountRepo.FindByID(entry.AccountID)
		if err != nil {
			return fmt.Errorf("invalid account_id %d: %w", entry.AccountID, err)
		}
		if account == nil {
			return fmt.Errorf("account with id %d does not exist", entry.AccountID)
		}
	}
	return nil
}