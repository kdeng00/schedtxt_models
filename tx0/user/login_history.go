package user

import (
	"github.com/google/uuid"
	"time"
)

type UserLoginHistory struct {
	Id        uuid.UUID `json:"id"`
	LoginTime time.Time `json:"login_time"`
	UserId    uuid.UUID `json:"user_id"`
}

type ServiceUserLoginHistory struct {
	Id            uuid.UUID `json:"id"`
	LoginTime     time.Time `json:"login_time"`
	ServiceUserId uuid.UUID `json:"service_user_id"`
}
