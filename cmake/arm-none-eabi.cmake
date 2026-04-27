# ARM Cortex-M cross-compilation toolchain for CMake
# Expects arm-none-eabi-gcc and related tools in ./toolchain/bin/

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR cortex-m4)

# Toolchain root directory (downloaded by gcc-toolchain.sh)
set(TOOLCHAIN_ROOT "${CMAKE_SOURCE_DIR}/toolchain")

# Compilers and tools
set(CMAKE_C_COMPILER "${TOOLCHAIN_ROOT}/bin/arm-none-eabi-gcc")
set(CMAKE_CXX_COMPILER "${TOOLCHAIN_ROOT}/bin/arm-none-eabi-g++")
set(CMAKE_ASM_COMPILER "${TOOLCHAIN_ROOT}/bin/arm-none-eabi-gcc")
set(CMAKE_OBJCOPY "${TOOLCHAIN_ROOT}/bin/arm-none-eabi-objcopy")
set(CMAKE_OBJDUMP "${TOOLCHAIN_ROOT}/bin/arm-none-eabi-objdump")
set(CMAKE_SIZE "${TOOLCHAIN_ROOT}/bin/arm-none-eabi-size")
set(CMAKE_STRIP "${TOOLCHAIN_ROOT}/bin/arm-none-eabi-strip")

# Configure for bare-metal (no OS)
set(CMAKE_C_FLAGS_INIT "-fno-common -ffunction-sections -fdata-sections")
set(CMAKE_EXE_LINKER_FLAGS_INIT "-specs=nano.specs -specs=nosys.specs -Wl,--gc-sections")

# Skip compiler checks that assume hosted environment
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)
