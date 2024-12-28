package resolver

import (
	"chotsuzuri/service"
)

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Resolver struct {
	journalService *service.JournalService
	accountService *service.AccountService
}

func NewResolver(js *service.JournalService, as *service.AccountService) *Resolver {
	return &Resolver{
		journalService: js,
		accountService: as,
	}
}
