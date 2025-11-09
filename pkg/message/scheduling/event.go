package scheduling

import (
	"time"

	"github.com/google/uuid"
)

type ScheduledEvent struct {
	Id uuid.UUID `json:"id"`
	RecipientId uuid.UUID `json:"recipient_id"`
	MessageId uuid.UUID `json:"message_id"`
	ScheduledMessageId uuid.UUID `json:"scheduled_message_id"`
	Created time.Time `json:"created"`
}
