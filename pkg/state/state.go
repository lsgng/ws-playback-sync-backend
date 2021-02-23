package state

type State struct {
	Playing bool
}

func NewState() *State {
	return &State{
		Playing: false,
	}
}
