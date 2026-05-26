package user

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInitUserProfile(t *testing.T) {
	usr := User{Username: "Bob", PhoneNumber: "+12224572351", Password: "TheNight!"}

	profile, done := InitUserProfile(&usr)

	assert.Equal(t, done, true, "Error")
	assert.NotEmpty(t, profile, "UserProfile is empty")
}
