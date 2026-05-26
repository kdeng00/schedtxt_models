package user


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
