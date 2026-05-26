package user

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInitUserProfile(t *testing.T) {
	usr := User{}
	usr.Username = "Bob"
	usr.PhoneNumber = "+12224572351"
	usr.Password = "TheNight!"

	profile, done := InitUserProfile(&usr)

	assert.Equal(t, done, true, "Error")
	assert.NotEmpty(t, profile, "UserProfile is empty")
}
