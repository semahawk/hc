# hc

A handy calculator. Think `bc` but more tailored for low-level operations such as bitshifts, bitwise XORs &c.

## Building

```
cargo build
```

## Usage

```
cargo run
```

This should drop you into the `hc` shell. Sample session:

```
>> 0b1100 | 0x3
res0 = 0000000015 (hex: 0000000f bin: 00000000000000000000000000001111)
>> 1 << 4 | res0
res1 = 0000000031 (hex: 0000001f bin: 00000000000000000000000000011111)
>> res1 + 1 | res1
res2 = 0000000063 (hex: 0000003f bin: 00000000000000000000000000111111)
>> 0'6
res3 = 0000000064 (hex: 00000040 bin: 00000000000000000000000001000000)
>> 0'6'7
res4 = 0000000192 (hex: 000000c0 bin: 00000000000000000000000011000000)
>> 0'6'7.6
res5 = 0000000128 (hex: 00000080 bin: 00000000000000000000000010000000)
>> zeropad = 16
res6 = 00016 (hex: 0010 bin: 0000000000010000)
>> 0'16 - 1
res7 = 65535 (hex: ffff bin: 1111111111111111)
```

## License

Copyright &copy; 2017 Szymon Urbaś

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
