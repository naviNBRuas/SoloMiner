cmake_minimum_required(VERSION 3.10)
project(SoloMiner VERSION 0.3.0 LANGUAGES C)

# SoloMiner CMake Wrapper
# This file allows the project to be integrated into larger CMake builds
# while still using Cargo for Rust compilation.

set(SOLOMINER_BIN_NAME "solo_miner")
if(WIN32)
    set(SOLOMINER_BIN_NAME "${SOLOMINER_BIN_NAME}.exe")
endif()

# Detect target architecture
if(NOT ARCH)
    if(CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|amd64|AMD64")
        set(ARCH "x86_64")
    elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|arm64")
        set(ARCH "aarch64")
    else()
        set(ARCH "unknown")
    endif()
endif()

message(STATUS "Building SoloMiner for ${CMAKE_SYSTEM_NAME} (${ARCH})")

# Cargo build command
find_program(CARGO_EXECUTABLE cargo REQUIRED)

add_custom_target(solominer_cargo_build ALL
    COMMAND ${CARGO_EXECUTABLE} build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building SoloMiner with Cargo"
)

# Install rules
install(PROGRAMS ${CMAKE_CURRENT_SOURCE_DIR}/target/release/${SOLOMINER_BIN_NAME}
        DESTINATION bin)

# Packaging configuration (CPack)
set(CPACK_PACKAGE_NAME "SoloMiner")
set(CPACK_PACKAGE_VENDOR "The Lonely Miner")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "A lonely miner without his pool.")
set(CPACK_PACKAGE_VERSION "${PROJECT_VERSION}")

if(WIN32)
    set(CPACK_GENERATOR "NSIS;WIX")
elseif(APPLE)
    set(CPACK_GENERATOR "DragNDrop")
else()
    set(CPACK_GENERATOR "DEB;RPM;TGZ")
    set(CPACK_DEBIAN_PACKAGE_MAINTAINER "lonely.miner@example.com")
endif()

include(CPack)
