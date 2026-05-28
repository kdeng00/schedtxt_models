package user

import (
	"time"

	"github.com/google/uuid"
)


type Organization struct {
	Id uuid.UUID `json:"id"`
	Name string `json:"name"`
	Created *time.Time `json:"created,omitempty"`
}
