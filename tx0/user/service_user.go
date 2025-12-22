package user

import (
	"time"

	"github.com/google/uuid"
)

type ServiceUser struct {
	Id         uuid.UUID `json:"id"`
	Username   string    `json:"username,omitempty"`
	Passphrase string    `json:"passphrase,omitempty"`
	Created    time.Time `json:"date_created"`
}
