package event

import (
	"time"

	"github.com/google/uuid"
)

type MessageEventResponse struct {
	Id                      uuid.UUID      `json:"id,omitempty"`
	ScheduledMessageEventId uuid.UUID      `json:"scheduled_message_event_id,omitempty"`
	Response                map[string]any `json:"response"`
	UserId                  uuid.UUID      `json:"user_id,omitempty"`
	ContactId               *uuid.UUID     `json:"contact_id,omitempty"`
	MessageId               *uuid.UUID     `json:"message_id,omitempty"`
	Status                  string         `json:"status"`
	Sent                    time.Time      `json:"sent"`
}
