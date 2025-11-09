package scheduling

import (
	"time"

	"github.com/google/uuid"
)


type ScheduledMessage struct {
	Id uuid.UUID `json:"id"`
	Scheduled time.Time `json:"scheduled"`
	Created time.Time `json:"created"`
	Status string `json:"status"`
	UserId uuid.UUID `json:"user_id"`
}
