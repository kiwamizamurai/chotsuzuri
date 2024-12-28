package domain

type Account struct {
    ID          uint        `gorm:"primaryKey" json:"id"`
    Code        string      `gorm:"unique;not null" json:"code"`
    Name        string      `gorm:"not null" json:"name"`
    AccountType AccountType `gorm:"not null" json:"account_type"`
}

type AccountType string

const (
    AccountTypeAsset     AccountType = "ASSET"
    AccountTypeLiability AccountType = "LIABILITY"
    AccountTypeEquity    AccountType = "EQUITY"
    AccountTypeRevenue   AccountType = "REVENUE"
    AccountTypeExpense   AccountType = "EXPENSE"
)