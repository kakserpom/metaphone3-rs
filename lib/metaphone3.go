package main

/*
#include <stdlib.h>

typedef struct {
    char* primary;
    char* secondary;
} MetaphoneResult;
*/
import "C"

import (
	"github.com/dlclark/metaphone3"
	"unsafe"
)

//export EncodeMetaphone
func EncodeMetaphone(cword *C.char, encode_vowels bool, encode_exact bool) C.MetaphoneResult {
    word := C.GoString(cword)
    encoder := &metaphone3.Encoder{EncodeVowels: encode_vowels, EncodeExact: encode_exact}
    prim, sec := encoder.Encode(word)

    var result C.MetaphoneResult
    result.primary = C.CString(prim)
    result.secondary = C.CString(sec)
    return result
}

//export FreeMetaphoneResult
func FreeMetaphoneResult(result C.MetaphoneResult) {
	C.free(unsafe.Pointer(result.primary))
	C.free(unsafe.Pointer(result.secondary))
}

func main() {}
