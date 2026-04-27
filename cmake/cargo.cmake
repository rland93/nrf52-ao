# CMake integration for Rust/Cargo
# Provides add_rust_staticlib() function to build Rust code and link it as a static library

# Find Cargo and Rustc
find_program(CARGO cargo REQUIRED)
find_program(RUSTC rustc REQUIRED)

function(add_rust_staticlib TARGET)
    cmake_parse_arguments(
        RUST
        ""
        "CRATE_NAME;CRATE_ROOT;TARGET"
        ""
        ${ARGN}
    )

    if(NOT RUST_CRATE_NAME)
        message(FATAL_ERROR "add_rust_staticlib: CRATE_NAME is required")
    endif()
    if(NOT RUST_CRATE_ROOT)
        message(FATAL_ERROR "add_rust_staticlib: CRATE_ROOT is required")
    endif()
    if(NOT RUST_TARGET)
        message(FATAL_ERROR "add_rust_staticlib: TARGET is required")
    endif()

    # Convert dashes to underscores (Rust convention for library names)
    string(REPLACE "-" "_" RUST_LIB_NAME "${RUST_CRATE_NAME}")

    set(CARGO_BUILD_DIR "${CMAKE_BINARY_DIR}/cargo")
    set(RUST_LIB "${CARGO_BUILD_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX}")

    # Determine build mode based on CMake configuration
    if(CMAKE_BUILD_TYPE STREQUAL "Release")
        set(CARGO_ARGS --release)
        set(CARGO_OUTPUT_DIR "release")
    else()
        set(CARGO_ARGS "")
        set(CARGO_OUTPUT_DIR "debug")
    endif()

    # Build Rust library with Cargo
    set(CARGO_LIB "${CARGO_BUILD_DIR}/${RUST_TARGET}/${CARGO_OUTPUT_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX}")

    add_custom_command(
        OUTPUT "${RUST_LIB}"
        COMMAND ${CMAKE_COMMAND} -E env
            "CARGO_TARGET_DIR=${CARGO_BUILD_DIR}"
            "RUSTFLAGS=-C target-feature=+crt-static"
            ${CARGO} build
                --manifest-path "${RUST_CRATE_ROOT}/Cargo.toml"
                --target ${RUST_TARGET}
                ${CARGO_ARGS}
        COMMAND ${CMAKE_COMMAND} -E copy
            "${CARGO_LIB}"
            "${RUST_LIB}"
        WORKING_DIRECTORY "${RUST_CRATE_ROOT}"
        DEPENDS "${RUST_CRATE_ROOT}/Cargo.toml" "${RUST_CRATE_ROOT}/Cargo.lock"
        COMMENT "Building Rust library: ${RUST_CRATE_NAME}"
    )

    # Create an imported static library target
    add_library(${TARGET} STATIC IMPORTED GLOBAL)
    set_target_properties(${TARGET} PROPERTIES
        IMPORTED_LOCATION "${RUST_LIB}"
    )
    add_custom_target(${TARGET}_build ALL DEPENDS "${RUST_LIB}")
    add_dependencies(${TARGET} ${TARGET}_build)
endfunction()
