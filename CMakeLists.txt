cmake_minimum_required(VERSION 3.15)

# ===== 工具链设置（必须在 project 前）=====
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv32)

find_program(RISCV_AS riscv64-unknown-elf-as)
find_program(RISCV_LD riscv64-unknown-elf-ld)
find_program(RISCV_OBJCOPY riscv64-unknown-elf-objcopy)

if(NOT RISCV_AS OR NOT RISCV_LD OR NOT RISCV_OBJCOPY)
    message(FATAL_ERROR "RISC-V toolchain not found.")
endif()

set(CMAKE_ASM_COMPILER ${RISCV_AS})
set(CMAKE_ASM_FLAGS "-march=rv32im -mabi=ilp32")
set(CMAKE_LINKER ${RISCV_LD})
set(CMAKE_EXE_LINKER_FLAGS "-m elf32lriscv -Ttext=0x80000000")

# 避免 C 编译器检查
set(CMAKE_C_COMPILER ${RISCV_AS})
set(CMAKE_C_COMPILER_WORKS TRUE)
set(CMAKE_C_COMPILER_FORCED TRUE)

project(riscv32-hello-world LANGUAGES ASM)

# --- 源文件 ---
set(SOURCES startup.S)

# --- 可执行文件（不加 .elf 后缀）---
add_executable(${PROJECT_NAME} ${SOURCES})

set_target_properties(${PROJECT_NAME} PROPERTIES
    LINKER_LANGUAGE ASM
    SUFFIX ".elf"
)

# --- 使用 generator expression 获取 ELF 路径 ---
set(ELF_FILE "$<TARGET_FILE:${PROJECT_NAME}>")

# --- 生成 .bin ---
add_custom_command(
    OUTPUT ${CMAKE_BINARY_DIR}/${PROJECT_NAME}.bin
    COMMAND ${RISCV_OBJCOPY} -O binary ${ELF_FILE} ${CMAKE_BINARY_DIR}/${PROJECT_NAME}.bin
    DEPENDS ${PROJECT_NAME}
    COMMENT "Generating binary image"
)

add_custom_target(${PROJECT_NAME}.bin ALL
    DEPENDS ${CMAKE_BINARY_DIR}/${PROJECT_NAME}.bin
)

# --- QEMU run target ---
find_program(QEMU_RISCV32 qemu-system-riscv32)
if(QEMU_RISCV32)
    add_custom_target(run
        COMMAND ${QEMU_RISCV32}
            -machine virt
            -nographic
            -bios none
            -serial mon:stdio
            -kernel ${ELF_FILE}
        DEPENDS ${PROJECT_NAME}
        COMMENT "Running in QEMU..."
    )
else()
    message(WARNING "qemu-system-riscv32 not found. 'run' target disabled.")
endif()

set_directory_properties(PROPERTIES ADDITIONAL_CLEAN_FILES "${PROJECT_NAME}.bin")