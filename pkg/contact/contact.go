package contact

import (
	"github.com/google/uuid"
)

type Contact struct {
	PhoneNumber string    `json:"phone_number,omitempty"`
	UserId      uuid.UUID `json:"user_id,omitempty"`
}
