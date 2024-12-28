package repository

import (
    "chotsuzuri/domain"
    "gorm.io/gorm"
)

type AccountRepository struct {
    db *gorm.DB
}

func NewAccountRepository(db *gorm.DB) *AccountRepository {
    return &AccountRepository{db: db}
}

func (r *AccountRepository) FindAll() ([]domain.Account, error) {
    var accounts []domain.Account
    err := r.db.Find(&accounts).Error
    return accounts, err
}

func (r *AccountRepository) FindByID(id uint) (*domain.Account, error) {
    var account domain.Account
    err := r.db.First(&account, id).Error
    if err != nil {
        return nil, err
    }
    return &account, nil
}