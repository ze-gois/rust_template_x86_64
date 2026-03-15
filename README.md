```needed are cc and ld```

```
pacman -S gcc llvm
```

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
mkdir hub
cd hub
git clone https://github.com/ze-gois/rust_template_x86_64_unix
cd rust_template_x86_64_unix
rustup target add x86_64-unknown-none
```

### student note

linux's syscalls are met by:

```
cd ..
git clone --depth=1 https://github.com/torvalds/linux
cd linux
git grep "SYSCALL_DEFINE"
```
