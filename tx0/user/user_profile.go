package user


type UserProfile struct {
	Usr *User `json:"user"`
}

func InitUserProfile(usr *User) (usrProfile *UserProfile, done bool) {
	if usr == nil {
		return nil, false
	} else {
		usrProfile = new(UserProfile)
		if len(usr.Password) > 0 {
			newUsr := User(Id: usr.Id, PhoneNumber: usr.PhoneNumber, Username: usr.Username, Firstname: usr.Firstname, Lastname: usr.Lastname, Created: usr.Created, LastLogin: usr.LastLogin)
			usrProfile.Usr = &newUsr

			return usrProfile, true
		} else {
			usrProfile.Usr = usr
			return usrProfile, true
		}
	}
}

