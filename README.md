# Nes
A emulator for **NES (Nintendo Entertainiment System)** made in Rust capable of run games like:  
	- **PacMan  
	- Donkey Kong  
    	- Super Mario Bros  
    	- etc.**  

---

# Installation

In Debian-based distros, run:
```sh
sudo apt install git cargo
git clone https://github.com/DevAles/nes.git
cd nes
chmod +x ./debian-install
./debian-install
```

In Arch-based distros, run:
```sh
yay -S git rust
git clone https://github.com/DevAles/nes.git
cd nes
chmod +x ./arch-install
./arch-install
```

---

# Running

```sh
cargo run
```

---

# References

Some useful links that i use to build this emulator:  
	- [6502 Instruction Reference](https://web.archive.org/web/20210428044647/http://www.obelisk.me.uk/6502/reference.html)  
	- [6502 Assembly Reference](https://en.wikibooks.org/wiki/6502_Assembly)
