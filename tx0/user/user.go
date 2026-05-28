package user

import (
	"time"

	"github.com/google/uuid"
)

type User struct {
	Id          uuid.UUID `json:"id"`
	PhoneNumber string    `json:"phone_number"`
	Username    string    `json:"username"`
	Password    string    `json:"password"`
	// Firstname is a pointer
	Firstname *string `json:"first_name,omitempty"`
	// Lastname is a pointer
	Lastname  *string    `json:"last_name,omitempty"`
	Created   time.Time  `json:"created"`
	LastLogin *time.Time `json:"last_login,omitempty"`
}
