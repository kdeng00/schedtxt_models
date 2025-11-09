package user

import (
	"github.com/google/uuid"
)

type User struct {
	Id          uuid.UUID `json:"id"`
	PhoneNumber string    `json:"phone_number"`
	Username    string    `json:"username"`
	Password    string    `json:"password"`
}
