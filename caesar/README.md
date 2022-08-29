## 🏛 Caesar Cipher CLI 🏛

This is an implementation of the historic [caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher). This serves mostly as an academic
exercise which helped me to learn many of the Rust topics. In order to maximize learnings, a decision to just use the standard library was
made.

⚠ **Important**: Users are encouraged to generally use modern cryptography instead ⚠

<br>
<p align="left">
<img src="https://upload.wikimedia.org/wikipedia/commons/4/4a/Caesar_cipher_left_shift_of_3.svg" alt="caesar" width="500"/>

<img src="https://upload.wikimedia.org/wikipedia/commons/b/b5/CipherDisk2000.jpg" alt="caesar" width="250"/>
</p>

<p align="right" style="color:silver">
Images by <a href="https://en.wikipedia.org/wiki/Caesar_cipher">Wikipedia</a>
</p>

### Features

* Only supports ascii alphabet. Other characters will be just omitted for processing, but included in the output.
* Encrypt from stdin.
* Encrypt from an input file.
* Output will be to the `stdout` by default. Users can also specify an output file.
* It supports overflow (more than 26 shifts) for the key. Max is a 6-digit number.
* It loads everything into memory. No streaming support, yet.

### How to install

You will need a [Rust installation](https://www.rust-lang.org/tools/install) as prerequisite.

Then just clone and install with:

```bash
git clone https://github.com/eloylp/rust-lab.git
cargo --install rust-lab/caesar
```

Great ! now you can execute `caesar -h`

```bash
$ caesar -h

🏛 Caesar Cipher 🏛

WARNING: Users are encouraged to use modern cryptography instead of this tool.
This was made for academic purposes with ❤ 🦀

Only -k argument is mandatory. If no other argument is provided stdin/stdout and
encryption mode are assumed.

Arguments:

-h     Shows this menu.
-v     Shows the version.
-k     The key, or positive shift number of the cipher (mandatory). Max is a 6 digit number.
-o     Write results to specified file.
-i     Specify path to input file.
-e     Encryption mode. (default).
-d     Decryption mode.

Here's a full example command:

$ caesar -k 10 -i input.txt -o output.txt -e
```

### Common usages

#### Reading from stdin, write to stdout

For encryption:

```bash
$ echo "ABC" | caesar -k 1
BCD
```

For decryption:

```bash
$ echo "BCD" | caesar -k 1 -d
ABC
```

#### Reading from input file, write to stdout

For encryption:

```bash
$ echo "ABC" > sample.txt
$ caesar -i sample.txt -k 1
BCD
```

For decryption:

```bash
$ echo "BCD" > encrypted.txt
$ caesar -i encrypted.txt -k 1 -d
ABC
```

### Reading from input file, write result to output file

For encryption:

```bash
$ echo "ABC" > plain.txt
$ caesar -k 1 -i plain.txt -o encrypted.txt
$ cat encrypted.txt
BCD
```

For decryption:

```bash
$ echo "BCD" > encrypted.txt
$ caesar -k 1 -d -i encrypted.txt -o plain.txt
$ cat plain.txt
ABC
```