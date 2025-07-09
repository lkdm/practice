package main

import (
    "io"
    "log"
    "net/http"
    "os"
    "fmt"
)

func main() {
	
    token := os.Getenv("TOL_TOKEN")
    if token == "" {
    	// Developer error
        panic("TOL_TOKEN environment variable not set")
    }
    bearer := "Bearer " + token

    req, err := http.NewRequest("GET", "https://rc.trionline.com.au/api/auth/", nil)
    if err != nil {
        panic(fmt.Errorf("SBRC CloneFacility unsuccessful due to problem forming request object: %s\n%s", err))
    }
    req.Header.Set("Authorization", bearer)

	client := &http.Client{
    	CheckRedirect: func(req *http.Request, via []*http.Request) error {
        	return http.ErrUseLastResponse
    	},
	}

    resp, err := client.Do(req)
    if err != nil {
        panic(fmt.Errorf("SBRC CloneFacility unsuccessful due to problem sending GET request: %s\n\n%s", err))
    }
    defer resp.Body.Close()

    body, err := io.ReadAll(resp.Body)
    if err != nil {
        panic(fmt.Errorf("SBRC CloneFacility unsuccessful due to problem parsing body response: %s\n\n%s", err, resp.Body))
    }

	// Check for an error code of some kind
	isHTTPError := resp.StatusCode >= 400 && resp.StatusCode < 600
	if isHTTPError {
		panic(fmt.Errorf("SBRC CloneFacility unsuccessful due to non-good HTTP code: %s\n\n%s", resp.Status, body))
	}

    log.Println(string(body))
}
