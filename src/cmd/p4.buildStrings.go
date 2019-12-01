//  ._______.___________________.___________________.___________________.___________________.___________________.___________________.___________________.                   .                   .                   .                   .                   ;
    package main

    import ( "fmt"; "bytes")

    const ESC byte = 0xBC
    const EOF byte = 0x00;

//  ._______.___________________.___________________.___________________.___________________.___________________._______;
    func mkString1 (rr string, nn int) (string) {

        var str string

        for ii := 0; ii < nn; ii++ {
           str += rr
        }
        return str
    }


//  ._______.___________________.___________________.___________________.___________________.___________________._______;
    func mkString2 (rr rune, nn int) (string, error) {

        var buf bytes.Buffer

        for ii := 0; ii < nn; ii++ {
            buf.WriteRune(rr)
        }

        str, err := buf.ReadString(EOF)
        return str, err
    }


//  ._______.___________________.___________________.___________________.___________________.___________________._______;
    func mkString3 (rr string, nn int) (string, error) {

        var buf bytes.Buffer

        for ii := 0; ii < nn; ii++ {
            buf.WriteString(rr)
        }
        buf.WriteByte(EOF)


        str, err := buf.ReadString(EOF)
        return str, err
    }


//  ._______.___________________.___________________.___________________.___________________.___________________._______;


//  ._______.___________________.___________________.___________________.___________________.___________________._______;
//  MAIN PROGRAM
    func main() {

        str := mkString1("a", 10)
        fmt.Printf("LEN: %d    STR: %s \n\n", len(str), str)

        str, err := mkString2('🈳', 20)
        fmt.Printf("LEN: %d   Error: %s   STR: %s \n\n", len(str), err, str)

        str, err = mkString3("🈴🈳", 10)
        fmt.Printf("LEN: %d   Ertrr: %s   STR: %s \n\n", len(str), err, str)
    }

//  🈳 🈴
