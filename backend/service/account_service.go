package service

import (
	"chotsuzuri/domain"
	"chotsuzuri/repository"
)

type AccountService struct {
	repo *repository.AccountRepository
}

func NewAccountService(repo *repository.AccountRepository) *AccountService {
	return &AccountService{repo: repo}
}

func (s *AccountService) ListAccounts() ([]domain.Account, error) {
	return s.repo.FindAll()
}

func (s *AccountService) GetAccount(id uint) (*domain.Account, error) {
	return s.repo.FindByID(id)
}