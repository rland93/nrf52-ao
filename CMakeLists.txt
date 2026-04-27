cmake_minimum_required(VERSION 3.20)

set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/cmake/arm-none-eabi.cmake"
    CACHE FILEPATH "ARM toolchain file")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


project(nrf52-ao C ASM)

set(CMAKE_C_STANDARD 11)

set(MCU_FLAGS
    -mcpu=cortex-m4
    -mthumb
    -mfloat-abi=hard
    -mfpu=fpv4-sp-d16
)

add_compile_options(
    ${MCU_FLAGS}
    -ffunction-sections
    -fdata-sections
    -fno-common
    -Wall
    -Wextra
    -g3
)

add_compile_definitions(
    NRF52840_XXAA
)

# --- NRFX (submodule) ---------------------------------------------------------
set(NRFX_DIR "${CMAKE_SOURCE_DIR}/vendor/nrfx")
set(NRFX_MDK "${NRFX_DIR}/bsp/stable/mdk")
set(NRFX_BSP_SOC "${NRFX_DIR}/bsp/stable/soc")
set(NRFX_BSP_TEMPLATES "${NRFX_DIR}/bsp/stable/templates")
set(CMSIS_CORE "${CMAKE_SOURCE_DIR}/vendor/CMSIS_6/CMSIS/Core/Include")
if(NOT EXISTS "${NRFX_MDK}/gcc_startup_nrf52840.S" OR NOT EXISTS "${CMSIS_CORE}/core_cm4.h")
  message(FATAL_ERROR
        "Submodules missing. Run:\n"
        "    git submodule update --init --recursive"
    )
endif()

set(NRFX_SOURCES
    ${NRFX_MDK}/gcc_startup_nrf52840.S
    ${NRFX_MDK}/system_nrf52840.c
    ${NRFX_DIR}/drivers/src/nrfx_egu.c
    ${NRFX_DIR}/drivers/src/nrfx_timer.c
)

# --- Firmware ELF -------------------------------------------------------------
add_executable(nrf52-ao.elf
    c/src/hw_egu.c
    c/src/hw_gpio.c
    c/src/hw_timer.c
    c/src/isr_mask.c
    c/src/main.c
    ${NRFX_SOURCES}
)

target_include_directories(nrf52-ao.elf PRIVATE
    c/include
    ${NRFX_DIR}
    ${NRFX_DIR}/bsp/stable
    ${NRFX_MDK}
    ${NRFX_BSP_SOC}
    ${NRFX_BSP_TEMPLATES}
    ${NRFX_DIR}/templates
    ${NRFX_DIR}/hal
    ${NRFX_DIR}/drivers/include
    ${NRFX_DIR}/drivers
    ${CMSIS_CORE}
)

# --- Rust staticlib -----------------------------------------------------------
include(${CMAKE_SOURCE_DIR}/cmake/cargo.cmake)
add_rust_staticlib(rust_app
    CRATE_NAME nrf52-ao
    CRATE_ROOT ${CMAKE_SOURCE_DIR}
    TARGET thumbv7em-none-eabihf
)
target_link_libraries(nrf52-ao.elf PRIVATE rust_app)

# --- Link ---------------------------------------------------------------------
set(LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/c/ld/nrf52840_app.ld")
target_link_options(nrf52-ao.elf PRIVATE
    ${MCU_FLAGS}
    -T${LINKER_SCRIPT}
    -Wl,-Map=${CMAKE_BINARY_DIR}/nrf52-ao.map
    -Wl,--gc-sections
    -Wl,--print-memory-usage
    -Wl,-z,noexecstack
)
set_target_properties(nrf52-ao.elf PROPERTIES
    LINK_DEPENDS ${LINKER_SCRIPT}
)

# --- Post-build: hex + size ---------------------------------------------------
add_custom_command(TARGET nrf52-ao.elf POST_BUILD
    COMMAND ${CMAKE_OBJCOPY} -O ihex $<TARGET_FILE:nrf52-ao.elf>
            ${CMAKE_BINARY_DIR}/nrf52-ao.hex
    COMMAND ${CMAKE_SIZE} $<TARGET_FILE:nrf52-ao.elf>
    COMMENT "Generating .hex and size report"
)
