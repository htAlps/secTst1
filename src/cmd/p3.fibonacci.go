//  ._______.___________________.___________________.___________________.___________________.___________________.___________________.___________________.                   .                   .                   .                   .                   ;
    package main

    import ( "fmt"; )
    type CxScanMap map[string] string                                   //  a Map of pending Scans 

    type FibMap map[int64] int64

    var kvs = make(FibMap)
//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  Fibonacci Function 
    func fib(nn int64) (int64) {

        if nn == 0 { return 0 }
        if nn < 3  { return 1 }
        if vv, ok := kvs[nn]; ok {
            return vv
        }
        resp := fib(nn-1) + fib(nn-2)
        kvs[nn] = resp
        return resp
    }

//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  Fibonacci Function 
    func printZeroVals() {
        var i int
        var f float64
        var b bool
        var s string
        fmt.Printf("%v %v %v %q\n", i, f, b, s)
    }

//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  MAIN PROGRAM
    func main() {
        printZeroVals()

        fmt.Printf("Index   Fib  (Fib/3)\n")
        for ii := 2; ii < 111; ii++ {
            ii64 := int64(ii); qq := fib(ii64); rr := float64(qq)/3.0;
            fmt.Printf("%2d - %7.2f  (%d)\n",ii, rr, qq)
        }
    }

