package user

import (
	"time"

	"github.com/google/uuid"
)

type ServiceUser struct {
	Id         uuid.UUID  `json:"id"`
	Passphrase string     `json:"passphrase,omitempty"`
	Created    *time.Time `json:"date_created,omitempty"`
}
