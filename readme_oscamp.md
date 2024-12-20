执行 `make run` 可以看到结果

```shell
... ...
Load payload ok!
Execute app ...
QEMU: Terminated
```



payload/apps.bin 中的内容，由

```c
#include "puts.h"

int main(int argc, char**argv)
{
//      hello();
        return 0;
}
```

编译而成。

编译过程：

```shell
#!/bin/bash

riscv64-linux-musl-gcc main.c -L. -lputs --static -no-pie -O2
riscv64-linux-musl-strip --strip-debug a.out
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary a.out ./hello_app.bin
dd if=/dev/zero of=./apps.bin bs=1M count=32
dd if=./hello_app.bin of=./apps.bin conv=notrunc
mv ./apps.bin ../arceos/payload/apps.bin
```



`riscv64-linux-musl-gcc` 中的 `crt1.c` 文件内容为：

```c

#include "crt_arch.h"

int main();
weak void _init();
weak void _fini();
int __libc_start_main(int (*)(), int, char **,
	void (*)(), void(*)(), void(*)());

unsigned long volatile abi_table = 0;

hidden void _start_c(long *p)
{
	abi_table = *p;
	p++;
	int argc = p[0];
	char **argv = (void *)(p+1);
	// __libc_start_main(main, argc, argv, _init, _fini, 0);
	main(argc, argv, 0);
}
```

