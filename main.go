package main

import (
	"fmt"
	"log"
	"net/http"

	websocket "b2b-prototype-backend/pkg/websocket"
)

func connectWebsocket(pool *websocket.Pool, w http.ResponseWriter, r *http.Request) {
	conn, err := websocket.Upgrade(w, r)
	if err != nil {
		fmt.Fprintf(w, "%+V\n", err)
	}

	client := &websocket.Client{
		Conn: conn,
		Pool: pool,
	}

	pool.Register <- client
	client.Read()
}

func setupRoutes() {
	pool := websocket.NewPool()
	go pool.Start()

	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
		connectWebsocket(pool, w, r)
	})
}

func main() {
	setupRoutes()
	err := http.ListenAndServe(":1234", nil)
	if err != nil {
		log.Fatal(err)
	}
}
