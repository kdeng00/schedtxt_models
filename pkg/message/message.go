package message

import (
	"github.com/google/uuid"
)

type Message struct {
	Id      uuid.UUID `json:"id"`
	Content string    `json:"content"`
	UserId  uuid.UUID `json:"user_id"`
}
