package contact

import (
	"github.com/google/uuid"
)

type Contact struct {
	Id          uuid.UUID `json:"id,omitempty"`
	PhoneNumber string    `json:"phone_number,omitempty"`
	UserId      uuid.UUID `json:"user_id,omitempty"`
}
