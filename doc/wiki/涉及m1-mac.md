* M1 MacbookPro 正常启动 Qemu 安装：
执行：
```
brew install ninja glib pixman pkg-config texinfo nettle gettext libffi
git clone https://github.com/patchew-project/qemu
cd qemu
```
然后需要加一个补丁，以避免qemu-system-riscv: qemu_mprotect__osdep: mprotect failed: Permission denied的问题。
```
// util/osdep.c 
// 在 int qemu_mprotect_none(void *addr, size_t size) 函数中加入 m1 平台判断

int qemu_mprotect_none(void *addr, size_t size)
{
#ifdef _WIN32
    return qemu_mprotect__osdep(addr, size, PAGE_NOACCESS);
#elif defined(__APPLE__) && defined(__arm64__)
    /* Workaround mprotect (RWX->NONE) issue on Big Sur 11.2 */
    return 0;
#else
    return qemu_mprotect__osdep(addr, size, PROT_NONE);
#endif
}
```
开始构建：
```
mkdir build && cd build
../configure --target-list=riscv64-softmmu,aarch64-softmmu --extra-cflags=-I/opt/homebrew/opt/gnutls/include --extra-ldflags=-L/opt/homebrew/opt/gnutls/lib --enable-cocoa
make -j
sudo make install
```
构建完毕，注意设置 qemu 的 PATH 为 build 目录。

以上。应该就可以在 M1 上正常使用 Qemu 了。
* [修改环境变量](https://www.jianshu.com/p/acb1f062a925)