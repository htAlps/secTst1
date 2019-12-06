//  ._______.___________________.___________________.___________________.___________________.___________________.___________________.___________________.                   .                   .                   .                   .                   ;
    package mux

    import ("os"; "io/ioutil"; "time"; "fmt"; "net/http"; "goji.io"; "goji.io/pat"; )

    const msg1      = `Hello World`
    const url       = `http://localhost:8000/hello/`
    const z_string  = ``
    const TTL_7sec  = 7   * time.Second
    const path_pk1  = `/usr/local/sys/bin/key/pk1.crt`
    const path_sk1  = `/usr/local/sys/bin/key/sk1.key`



//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  Basic http server 
    func handlHello(ww http.ResponseWriter, rr *http.Request) {

        name := pat.Param(rr, `name`)
        fmt.Fprintf(ww, "Hello %s, my friend", name)

    }


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
//  Basic http client
    func Main_cli() {

        body, err := get(`:Gandalf`, url)
        if err != nil {
            fmt.Printf("\nERROR: %v \nExiting ... \n", err.Error())
            os.Exit(1)
        }
        fmt.Printf("\nRESP:\n%v\n\n", body)
    }


//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  Basic http server 
    func Main_svc() {

        rr := goji.NewMux()
        rr.HandleFunc(pat.Get("/hello/:name"), handlHello)

        http.ListenAndServe("localhost:8000", rr)

    }


/*  CODE PIT
    const url       = `https://localhost:8000/hello/`
        http.ListenAndServeTLS("localhost:8000", path_pk1, path_sk1, rr)
        //  fmt.Printf("\n%s\n\n", msg1)
*/
