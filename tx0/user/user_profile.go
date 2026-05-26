package user

import (
	"fmt"
)

type UserProfile struct {
	Usr *User `json:"user"`
}

func InitUserProfile(usr *User) (usrProfile *UserProfile, err error) {
	if usr == nil {
		return nil, fmt.Errorf("User is empty")
	} else {
		usrProfile = new(UserProfile)
		if len(usr.Password) > 0 {
			newUsr := User{Id: usr.Id, PhoneNumber: usr.PhoneNumber, Username: usr.Username, Firstname: usr.Firstname, Lastname: usr.Lastname, Created: usr.Created, LastLogin: usr.LastLogin,}
			usrProfile.Usr = &newUsr

			return usrProfile, nil
		} else {
			usrProfile.Usr = usr
			return usrProfile, nil
		}
	}
}

