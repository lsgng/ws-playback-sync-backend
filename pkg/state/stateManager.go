package state

import (
	"fmt"

	websocket "b2b-prototype-backend/pkg/websocket"
)

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

func (stateManager *StateManager) Start(broadcast chan websocket.Message) {
	for {
		select {
		case event := <-stateManager.Event:
			fmt.Println("PROCESSING COMMAND ", event)
			broadcast <- websocket.Message{Type: 1, Body: "JUHUUUU"}
		}
	}
}
