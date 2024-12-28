package domain

import (
    "time"
)

type Journal struct {
    ID            uint          `gorm:"primaryKey"`
    JournalNumber string        `gorm:"unique;not null"`
    Date          time.Time     `gorm:"not null"`
    Description   string        `gorm:"not null"`
    Entries       []JournalEntry `gorm:"foreignKey:JournalID;constraint:OnDelete:CASCADE"`
    CreatedAt     time.Time
    UpdatedAt     time.Time
}

type JournalEntry struct {
    ID        uint    `gorm:"primaryKey"`
    JournalID uint    `gorm:"not null"`
    Journal   Journal `gorm:"foreignKey:JournalID"`
    AccountID uint    `gorm:"not null"`
    Account   Account `gorm:"foreignKey:AccountID"`
    IsDebit   bool    `gorm:"not null"`
    Amount    int     `gorm:"not null"`
}

type JournalFilter struct {
    DateRange struct {
        From *time.Time
        To   *time.Time
    }
    AccountCodes []string
    AmountRange struct {
        Min *int
        Max *int
    }
}

type DateRange struct {
    From *time.Time
    To   *time.Time
}

type AmountRange struct {
    Min *int
    Max *int
}