package user

import (
	"time"

	"github.com/google/uuid"
)

type User struct {
	Id          uuid.UUID  `json:"id"`
	PhoneNumber string     `json:"phone_number"`
	Username    string     `json:"username"`
	Password    string     `json:"password"`
	Firstname   *string    `json:"first_name,omitempty"`
	Lastname    *string    `json:"last_name,omitempty"`
	Created     time.Time  `json:"date_created"`
	LastLogin   *time.Time `json:"last_login,omitempty"`
}
