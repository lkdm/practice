package main

import (
	"log"
	"net/http"
	"os"
	"fmt"
)

func healthCheckHandler(w http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(w, "ok\n")
}



func apiHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello World!")
}

func setupHandlers(mux *http.ServeMux) {
	mux.HandleFunc("/health", healthCheckHandler)
	mux.HandleFunc("/api", apiHandler)
}

func main() {

	// Set up listening address
	listenAddr := os.Getenv("LISTEN_ADDR")
	if len(listenAddr) == 0 {
		listenAddr = ":8080"
	}

	// Set up handlers
	mux := http.NewServeMux()
	setupHandlers(mux)

	log.Fatal(http.ListenAndServe(listenAddr, mux))
}

