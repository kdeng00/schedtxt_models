package contact

import (
	"github.com/google/uuid"
)

type Contact struct {
	Id          *uuid.UUID `json:"id,omitempty"`
	Firstname   *string    `json:"first_name,omitempty"`
	Lastname    *string    `json:"last_name,omitempty"`
	Nickname    *string    `json:"nickname,omitempty"`
	PhoneNumber string     `json:"phone_number"`
	UserId      *uuid.UUID `json:"user_id,omitempty"`
}
