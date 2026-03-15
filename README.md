```needed are cc and ld```

```
pacman -S gcc llvm
```

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
mkdir hub
cd hub
git clone https://github.com/ze-gois/rust_template_x86_64
cd rust_template_x86_64
rustup target add x86_64-unknown-none
```
