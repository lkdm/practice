package main
import (
	"net/http"	
	"net/http/httptest"
	"testing"
	"io/ioutil"
)

func TestHealthCheckHandler(t *testing.T) {
	req := httptest.NewRequest("GET", "/health", nil)
	w := httptest.NewRecorder()

	healthCheckHandler(w, req)

	resp := w.Result()
	body, _ := ioutil.ReadAll(resp.Body)

	if resp.StatusCode != http.StatusOK {
		t.Fatalf("expected status 200 OK, got %d", resp.StatusCode)
	}
	if string(body) != "ok\n" {
		t.Fatalf("expected body 'ok\\n', got %q", string(body))
	}
}
