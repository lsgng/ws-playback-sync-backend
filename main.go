package main

import (
	"fmt"
	"log"
	"net/http"

	state "b2b-prototype-backend/pkg/state"
	websocket "b2b-prototype-backend/pkg/websocket"
)

func serveWebsocket(pool *websocket.Pool, w http.ResponseWriter, r *http.Request, eventChannel chan string) {
	conn, err := websocket.Upgrade(w, r)
	if err != nil {
		fmt.Fprintf(w, "%+V\n", err)
	}

	client := &websocket.Client{
		Conn: conn,
		Pool: pool,
	}

	pool.Register <- client
	client.Read(eventChannel)
}

func setupRoutes() {
	stateManager := state.NewStateManager()
	go stateManager.Start()

	pool := websocket.NewPool()
	go pool.Start()

	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
		serveWebsocket(pool, w, r, stateManager.Event)
	})
}

func main() {
	setupRoutes()
	err := http.ListenAndServe(":1234", nil)
	if err != nil {
		log.Fatal(err)
	}
}
