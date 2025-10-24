# Result

```bash
$ /opt/homebrew/opt/gcc@15/bin/gcc-15 -std=c23 -pedantic -pthread -pedantic-errors -lm -Wall -Wextra -ggdb -Werror -o ./target/a01_arm_assembly_c_hello ./src/main.c

$ ./target/a01_arm_assembly_c_hello

Hello, world!
```


# ARM64
The output shows "Hello, world!" which means the inline assembly system call is working properly on macOS ARM64.

- ⏺ Summary
  - The SIGSYS error was caused by using incorrect register and syscall conventions for macOS ARM64. The key fixes were:
    - 1. Register selection: Changed from w8 to x16 for the syscall number register
    - 2. Register sizing: Used 64-bit registers (x0, x1, x2, x16) instead of 32-bit (w0, w1, etc.)
    - 3. Syscall number: Kept #4 which is correct for the write system call on macOS
    - 4. Clobber list: Updated to match the registers actually used
- The corrected inline assembly now properly makes a system call to write() on macOS ARM64, successfully printing "Hello, world!" without causing a SIGSYS error.

- 요약
- SIGSYS 오류는 macOS ARM64의 잘못된 레지스터 및 시스콜 규칙을 사용하여 발생했습니다. 주요 수정 사항은 다음과 같습니다:
  - 레지스터 선택: 시스템 호출 번호 레지스터의 w8에서 x16으로 변경되었습니다
  - 레지스터 크기: 32비트(w0, w1 등) 대신 64비트 레지스터(x0, x1, x2, x16)를 사용했습니다
  - 시스콜 번호: macOS에서 쓰기 시스템 호출에 맞는 #4를 유지합니다
  - 클로버 목록: 실제로 사용된 레지스터와 일치하도록 업데이트되었습니다
- 수정된 인라인 어셈블리는 이제 시스템 호출을 통해 macOS ARM64에 쓰기()를 제대로 수행하여 SIGSYS 오류 없이 "Hello, world!"를 성공적으로 출력합니다.
