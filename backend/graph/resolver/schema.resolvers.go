package resolver

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.
// Code generated by github.com/99designs/gqlgen version v0.17.61

import (
	"context"
	"fmt"
	"chotsuzuri/domain"
	"chotsuzuri/graph/generated"
	"chotsuzuri/graph/model"
	"strconv"
)

func makeJournalEntries(entries []domain.JournalEntry) []*model.JournalEntry {
    result := make([]*model.JournalEntry, len(entries))
    for i, e := range entries {
        result[i] = &model.JournalEntry{
            ID:      strconv.FormatUint(uint64(e.ID), 10),
            Account: makeAccount(e.Account),
            IsDebit: e.IsDebit,
            Amount:  e.Amount,
        }
    }
    return result
}

func makeAccount(a domain.Account) *model.Account {
    return &model.Account{
        ID:          strconv.FormatUint(uint64(a.ID), 10),
        Code:        a.Code,
        Name:        a.Name,
        AccountType: model.AccountType(a.AccountType),
    }
}

// Journals is the resolver for the journals field.
func (r *queryResolver) Journals(ctx context.Context, filter *model.JournalFilter, pagination model.PaginationInput) (*model.JournalConnection, error) {
	domainFilter := &domain.JournalFilter{}

	if filter != nil {
		if filter.DateRange != nil {
			domainFilter.DateRange.From = filter.DateRange.From
			domainFilter.DateRange.To = filter.DateRange.To
		}
		domainFilter.AccountCodes = filter.AccountCodes
		if filter.AmountRange != nil {
			domainFilter.AmountRange.Min = filter.AmountRange.Min
			domainFilter.AmountRange.Max = filter.AmountRange.Max
		}
	}

	journals, total, err := r.journalService.ListJournals(pagination.Page, pagination.PerPage, domainFilter)
	if err != nil {
		return nil, err
	}

	items := make([]*model.Journal, len(journals))
	for i, j := range journals {
		items[i] = &model.Journal{
			ID:            strconv.FormatUint(uint64(j.ID), 10),
			JournalNumber: j.JournalNumber,
			Date:          j.Date,
			Description:   j.Description,
			Entries:       makeJournalEntries(j.Entries),
			CreatedAt:     j.CreatedAt,
			UpdatedAt:     j.UpdatedAt,
		}
	}

	totalPages := (int(total) + pagination.PerPage - 1) / pagination.PerPage

	return &model.JournalConnection{
		Items:      items,
		PageInfo: &model.PageInfo{
			HasNextPage: pagination.Page < totalPages,
			HasPrevPage: pagination.Page > 1,
			TotalPages:  totalPages,
			CurrentPage: pagination.Page,
		},
	}, nil
}

// Journal is the resolver for the journal field.
func (r *queryResolver) Journal(ctx context.Context, id string) (*model.Journal, error) {
	panic(fmt.Errorf("not implemented: Journal - journal"))
}

// Accounts is the resolver for the accounts field.
func (r *queryResolver) Accounts(ctx context.Context) ([]*model.Account, error) {
	panic(fmt.Errorf("not implemented: Accounts - accounts"))
}

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type queryResolver struct{ *Resolver }

// !!! WARNING !!!
// The code below was going to be deleted when updating resolvers. It has been copied here so you have
// one last chance to move it out of harms way if you want. There are two reasons this happens:
//  - When renaming or deleting a resolver the old code will be put in here. You can safely delete
//    it when you're done.
//  - You have helper methods in this file. Move them out to keep these resolver files clean.
/*
	func makeJournalEntries(entries []domain.JournalEntry) []*model.JournalEntry {
	result := make([]*model.JournalEntry, len(entries))
	for i, e := range entries {
		result[i] = &model.JournalEntry{
			ID:      strconv.FormatUint(uint64(e.ID), 10),
			Account: makeAccount(e.Account),
			IsDebit: e.IsDebit,
			Amount:  e.Amount,
		}
	}
	return result
}
func makeAccount(a domain.Account) *model.Account {
	return &model.Account{
		ID:          strconv.FormatUint(uint64(a.ID), 10),
		Code:        a.Code,
		Name:        a.Name,
		AccountType: model.AccountType(a.AccountType),
	}
}
*/
