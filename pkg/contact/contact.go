package contact

type Contact struct {
	PhoneNumber string `json:"phone_number,omitempty"`
	UserId      string `json:"user_id,omitempty"`
}
