package message

import (
	"time"

	"github.com/google/uuid"
)

type MessageEventResponse struct {
	Id uuid.UUID `json:"id,omitempty"`
	ScheduledMessageEventId uuid.UUID `json:"scheduled_message_event_id,omitempty"`
	Response []byte `json:"response"`
	UserId uuid.UUID `json:"user_id,omitempty"`
	Sent time.Time `json:"sent"`
}
