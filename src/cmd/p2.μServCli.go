//  ._______.___________________.___________________.___________________.___________________.___________________.___________________.___________________.                   .                   .                   .                   .                   ;
    package main

    import ( "fmt"; "net/http"; "os"; "io/ioutil"; "time"; )

    const msg1      = `Hello World`
    const url       = `https://localhost:8000/hello/`
    const z_string  = ``
    const TTL_7sec  = 7   * time.Second

//  CONSTANTS:  KEYS ONLY FOR P1I-SEC-ETCD01 GET SERVICE [sk7]
const sk = `-----BEGIN EC PARAMETERS-----
BgUrgQQAIw==
-----END EC PARAMETERS-----
-----BEGIN EC PRIVATE KEY-----
MIHcAgEBBEIA5GEFBshIkibR7Oyux8JyssNnNXw4dQxA438t6SvvzrtXfiaNmBsmGuN4R0thOkPFkHcuusw9hL4XzQ2DW3Xglb2gBwYFK4EEACOhgYkDgYYABAB/GTMJ+CgZ+HzPHA3iUewCXZqaDx9/1JjzSmd7BDEJJ0Cqoa9ysKYs1OxE4toVglo0YmvL4xlU0ES9W2UIYX+zLgGNWJK6BBWkiTD7b5TNqVYDvox3QLhh8W+j1qH7e2y+G5BT
nJheHYk3tUg8waV/jecup2QUiadFYW19uLMQAXY13A==
-----END EC PRIVATE KEY-----
`

const sCert = `-----BEGIN CERTIFICATE-----
MIIDGzCCAnygAwIBAgIUUbGmVzpwM4snl2govWQrskYm5TcwCgYIKoZIzj0EAwIwfTELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkZMMQ4wDAYDVQQHDAVNaWFtaTEMMAoGA1UECgwDSVBDMREwDwYDVQQLDAhTZWN1cml0eTEMMAoGA1UEAwwDQVBJMSIwIAYJKoZIhvcNAQkBFhNzZWN1cml0eUBpcGNvb3AuY29tMB4XDTE5MDUwOTIzMTYzNloX
DTI5MDYyNTIzMTYzNlowfTELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkZMMQ4wDAYDVQQHDAVNaWFtaTEMMAoGA1UECgwDSVBDMREwDwYDVQQLDAhTZWN1cml0eTEMMAoGA1UEAwwDQVBJMSIwIAYJKoZIhvcNAQkBFhNzZWN1cml0eUBpcGNvb3AuY29tMIGbMBAGByqGSM49AgEGBSuBBAAjA4GGAAQAfxkzCfgoGfh8zxwN4lHsAl2amg8ff9SY
80pnewQxCSdAqqGvcrCmLNTsROLaFYJaNGJry+MZVNBEvVtlCGF/sy4BjViSugQVpIkw+2+UzalWA76Md0C4YfFvo9ah+3tsvhuQU5yYXh2JN7VIPMGlf43nLqdkFImnRWFtfbizEAF2NdyjgZYwgZMwQAYDVR0RBDkwN4IOcDFpLXNlYy1ldGNkMDGCGXAxaS1zZWMtZXRjZDAxLmlwY29vcC5jb22HBAoVDsmHBH8AAAEwHQYDVR0OBBYEFDV0
MeLcxGfmqu02Y5itiizRPdDnMB8GA1UdIwQYMBaAFDV0MeLcxGfmqu02Y5itiizRPdDnMA8GA1UdEwEB/wQFMAMBAf8wCgYIKoZIzj0EAwIDgYwAMIGIAkIA/MUN767ef07sTSZnwyDZkRFpl0cnuMiINAj1okoFuyB97ZSRaluDkICmoX/KHSix3seAF9iESxQB7VW+ST/fE50CQgG3RFFO7TUQJh47KnMfA1T9xTs8ZQJGkjK7mpvGyF9hWOZ+
kbaZ9Z2Z5lJ2QKrpnOnVefo4ogir+dpOdMqhW/ZvmA==
-----END CERTIFICATE-----
`

//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  get gets a response from service 
    func get(what, fromUrl string) (string, error) {

        req, err := http.NewRequest(`GET`, fromUrl + what, nil)
        if err != nil {
            fmt.Printf("\n%v \nExiting ... \n", err.Error())
            return z_string, err
        }
        cli := &http.Client { Timeout: TTL_7sec, }

        resp, err := cli.Do(req)
        if err != nil {
            fmt.Printf("\nERROR: %v \nRESP: %v \nExiting ... \n", err.Error(), resp)
            return z_string, err
        }
        defer resp.Body.Close()
        fmt.Printf("\nRESP: \n%v\n\n", resp)

        body, err := ioutil.ReadAll(resp.Body)
        return string(body), err
    }


//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  MAIN PROGRAM
    func main() {

        body, err := get(`:Gandalf`, url)
        if err != nil {
            fmt.Printf("\nERROR: %v \nExiting ... \n", err.Error())
            os.Exit(1)
        }
        fmt.Printf("\nRESP:\n%v\n\n", body)
    }


/*  CODE PIT
*/

