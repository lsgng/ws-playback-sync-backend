package state

import "fmt"

type StateManager struct {
	State State
	Event chan string
}

func NewStateManager() *StateManager {
	return &StateManager{
		State: State{
			Playing: false,
		},
		Event: make(chan string),
	}
}

func (stateManager *StateManager) Start() {
	fmt.Println("starting channel")
	for {
		fmt.Println("loop")
		select {
		case event := <-stateManager.Event:
			fmt.Println("PROCESSING COMMAND ", event)
		}
	}
}
