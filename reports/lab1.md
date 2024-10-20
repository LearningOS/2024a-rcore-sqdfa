# **实现的功能** 

完成新的系统调用 `sys_task_info` 以获取当前任务的信息



# **简答** 

## 1

```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

sbi版本

```txt
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
```

## 2

### 2.1

`a0` 存储着内核栈的地址

两种使用情景：1）初始加入用户态；2）从S态返回用户态

### 2.2

`sstatus`：状态寄存器，保存陷阱返回后要恢复的状态。

`sepc`：异常程序计数器，保存返回用户态时执行的下一条指令的地址。

`sscratch`：临时寄存器，用来在陷阱处理期间暂时保存用户栈指针。

### 2.3

`x2`（栈指针）和 `x4`（线程指针）都有特殊的功能

### 2.4

恢复后，`sp` 将指向用户态栈，而 `sscratch` 保存了内核态栈的地址

### 2.5

使用 `sret` 指令回到 U 特权级继续运行应用程序控制流

### 2.7

应用程序通过 `ecall` 进入到内核状态