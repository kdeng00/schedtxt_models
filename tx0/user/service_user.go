package user

import (
	"time"

	"github.com/google/uuid"
)

type ServiceUser struct {
	Id uuid.UUID `json:"id"`
	// TODO: Remove the omitempty tags at a later point
	Username   string     `json:"username,omitempty"`
	Passphrase string     `json:"passphrase,omitempty"`
	Created    time.Time  `json:"created"`
	LastLogin  *time.Time `json:"last_login,omitempty"`
}
