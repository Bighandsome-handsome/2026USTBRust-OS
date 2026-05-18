当前输出：
```bash
# 前省略一些信息
Hello World!
run_first_task!
AAAAAAAAAA [1/5]
run_next_task: 1
run_next_task: 2
CCCCCCCCCC [1/5]
run_next_task: 3
Hello from C!
run_next_task: 4
Trace System Call Test
Test 1: Get initial syscall count
  SYSCALL_WRITE (64) count: 2
Test 2: Call write 3 times and check count
  Line 1
run_next_task: 5
Simple Trace Example
Checking write syscall count...
WRITE count: 2
Making 5 write calls:
  Write #1
  Write #2
  Write #3
  Write #4
  Write #5
WRITE count now: 21
Getting task information...
Total syscalls: 29
Task status: 2
Success trace_simple!
run_next_task: 0
run_next_task: 1
BBBBBBBBBB [1/5]
run_next_task: 2
CCCCCCCCCC [2/5]
run_next_task: 4
  Line 2
  Line 3
  SYSCALL_WRITE count completed after writes: 9
Test 4: Get task information
  Task Status: 2 (RUNNING) OK!
  Total Syscalls: run_next_task: 0
AAAAAAAAAA [2/5]
run_next_task: 1
BBBBBBBBBB [2/5]
run_next_task: 2
CCCCCCCCCC [3/5]
run_next_task: 4
16 OK!
run_next_task: 0
AAAAAAAAAA [3/5]
run_next_task: 1
BBBBBBBBBB [3/5]
run_next_task: 2
CCCCCCCCCC [4/5]
run_next_task: 4
Test 5: Check counts for different syscalls
  SYSCALL_EXIT (93) count: 0
  SYSCALL_YIELD (124) count completed: 1
  SYSCALL_TRACE (410) count completed: 6
Test 6: Test with invalid syscall ID
run_next_task: 0
AAAAAAAAAA [4/5]
run_next_task: 1
BBBBBBBBBB [4/5]
run_next_task: 2
CCCCCCCCCC [5/5]
run_next_task: 4
  Querying syscall 999, result: 0 (should be -1 or 0)
Test 7: Call yield() and check count
  SYSCALL_YIELD count before: 1
run_next_task: 0
AAAAAAAAAA [5/5]
run_next_task: 1
run_next_task: 2
Test write C OK!
run_next_task: 4
  SYSCALL_YIELD count completed after: 2
Test 8: Call get_time() and check count
  SYSCALL_GET_TIME count before: 0
  Current time: 0.185496 seconds
  SYSCALL_GET_TIME count after: 1
Test 9: Get total syscall count
  Success! Total syscalls: run_next_task: 0
Test write A OK!
run_next_task: 1
BBBBBBBBBB [5/5]
run_next_task: 4
75
Final Task Information
Success! Total syscalls made: 81
All trace syscall tests finished!
run_next_task: 1
Test write B OK!
[kernel] Panicked at src/task/mod.rs:144 All applications completed!
```