// mutowalność jest wbudowana w język

type S struct {
    A string
    B []string
}
 
func main() {
    x := S{"x-A", []string{"x-B"}}
    y := x // copy the struct
    y.A = "y-A"
    y.B[0] = "y-B"
 
    fmt.Println(x, y)
    // Outputs "{x-A [y-B]} {y-A [y-B]}" -- x was modified!
}

// slices i kwestia append

func doStuff(value []string) {
    fmt.Printf("value=%v\n", value)
 
    value2 := value[:]
    value2 = append(value2, "b")
    fmt.Printf("value=%v, value2=%v\n", value, value2)
 
    value2[0] = "z"
    fmt.Printf("value=%v, value2=%v\n", value, value2)
}
 
func main() {
    slice1 := []string{"a"} // length 1, capacity 1
 
    doStuff(slice1)
    // Output:
    // value=[a] -- ok
    // value=[a], value2=[a b] -- ok: value unchanged, value2 updated
    // value=[a], value2=[z b] -- ok: value unchanged, value2 updated
 
    slice10 := make([]string, 1, 10) // length 1, capacity 10
    slice10[0] = "a"
 
    doStuff(slice10)
    // Output:
    // value=[a] -- ok
    // value=[a], value2=[a b] -- ok: value unchanged, value2 updated
    // value=[z], value2=[z b] -- WTF?!? value changed???
}

// error handling

len, err := reader.Read(bytes)
if err != nil {
    if err == io.EOF {
        // All good, end of file
    } else {
        return err
    }
}


// interfejs nil

type Explodes interface {
    Bang()
    Boom()
}
 
// Type Bomb implements Explodes
type Bomb struct {}
func (*Bomb) Bang() {}
func (Bomb) Boom() {}
 
func main() {
    var bomb *Bomb = nil
    var explodes Explodes = bomb
    println(bomb, explodes) // '0x0 (0x10a7060,0x0)'
    if explodes != nil {
        println("Not nil!") // 'Not nil!' What are we doing here?!?!
        explodes.Bang()     // works fine
        explodes.Boom()     // panic: value method main.Bomb.Boom called using nil *Bomb pointer
    } else {
        println("nil!")     // why don't we end up here?
    }
}

// ubogie struktury danych, takie customowe tracą type safety m.in poprzez castowanie do interface{}
// kiedyś brak generyków, choć teraz w znacznym stopniu problem został rozwiązany.