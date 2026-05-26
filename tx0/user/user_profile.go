package user

import (
	"testing"

	"github.com/stretchr/testify/assert"
)


type UserProfile struct {
	Usr *User `json:"user"`
}

func InitUserProfile(usr *User) (usrProfile *UserProfile, done bool) {
	if usr == nil {
		return nil, false
	} else {
		if len(usr.Password) > 0 {
			var newUsr User
			newUsr = *usr
			newUsr.Password = ""
			usrProfile.Usr = &newUsr

			return usrProfile, true
		} else {
			usrProfile.Usr = usr
			return usrProfile, true
		}
	}
}

func TestInitUserProfile(t *testing.T) {
	usr := User{}
	usr.Username = "Bob"
	usr.PhoneNumber = "+12224572351"
	usr.Password = "TheNight!"
	profile, done := InitUserProfile(&usr)
	assert.Equal(t, done, true, "Error")
	assert.NotEmpty(t, profile, "UserProfile is empty")
}
