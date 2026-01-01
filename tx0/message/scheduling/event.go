package scheduling

import (
	"time"

	"github.com/google/uuid"
)

type ScheduledMessageEvent struct {
	Id                 uuid.UUID `json:"id"`
	ContactId          uuid.UUID `json:"contact_id"`
	MessageId          uuid.UUID `json:"message_id"`
	ScheduledMessageId uuid.UUID `json:"scheduled_message_id"`
	Created            time.Time `json:"created"`
}
