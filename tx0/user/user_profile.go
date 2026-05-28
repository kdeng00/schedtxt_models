package user

import (
	"fmt"
	"time"

	"github.com/google/uuid"
)

// TODO: Replace Usr with all the fields from User, except Password and other sensitive fields
type UserProfile struct {
	UserId uuid.UUID `json:"user_id"`
	PhoneNumber string `json:"phone_number"`
	Username string `json:"username"`
	Firstname string `json:"firstname"`
	Lastname string `json:"lastname"`
	Created time `json:"created"`
	LastLogin *time `json:"last_login,omitempty"`
}

func InitUserProfile(usr *User) (usrProfile *UserProfile, err error) {
	if usr == nil {
		return nil, fmt.Errorf("User is empty")
	} else {
		usrProfile = new(UserProfile)
		if len(usr.Password) > 0 {
			// newUsr := User{Id: usr.Id, PhoneNumber: usr.PhoneNumber, Username: usr.Username, Firstname: usr.Firstname, Lastname: usr.Lastname, Created: usr.Created, LastLogin: usr.LastLogin}
			// usrProfile.Usr = &newUsr
			usrProfile.UserId = usr.Id
			usrProfile.PhoneNumber = usr.PhoneNumber
			usrProfile.Username = usr.Username
			usrProfile.Firstname = *usr.Firstname
			usrProfile.Lastname = *usr.Lastname
			usrProfile.Created = usr.Created
			if usr.LastLogin != nil {
				usrProfile.LastLogin = usr.LastLogin
			}

			return usrProfile, nil
		} else {
			usrProfile.Usr = usr
			return usrProfile, nil
		}
	}
}
