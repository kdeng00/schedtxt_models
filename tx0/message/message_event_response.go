package message

import (
	"time"

	"github.com/google/uuid"

	"git.kundeng.us/phoenix/textsender-models/tx0/types"
)

type MessageEventResponse struct {
	Id                      uuid.UUID   `json:"id,omitempty"`
	ScheduledMessageEventId uuid.UUID   `json:"scheduled_message_event_id,omitempty"`
	Response                types.JSONB `json:"response"`
	UserId                  uuid.UUID   `json:"user_id,omitempty"`
	ContactId               *uuid.UUID  `json:"contact_id,omitempty"`
	MessageId               *uuid.UUID  `json:"message_id,omitempty"`
	Status                  string      `json:"status"`
	Sent                    time.Time   `json:"sent"`
}

const (
	Message_Event_Response_Status_Instant   = "INSTANT"
	Message_Event_Response_Status_Scheduled = "SCHEDULED"
)
