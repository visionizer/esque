// Known functions: syscall_dispatcher

.global syscall_handler
syscall_handler:
    push rax
    ret