cmake_minimum_required(VERSION 3.5)
project(wasp-input DESCRIPTION "wasp-input")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)  # Helps with symbol exports on Windows

add_library(${PROJECT_NAME} SHARED main.cpp exports.cpp)

# Set output names for different architectures
if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    set_target_properties(${PROJECT_NAME} PROPERTIES
        PREFIX ""
        OUTPUT_NAME "wasp-input64")
else()
    set_target_properties(${PROJECT_NAME} PROPERTIES
        PREFIX ""
        OUTPUT_NAME "wasp-input32")
endif()

# Platform-specific settings
if(WIN32)
    target_compile_definitions(${PROJECT_NAME} PRIVATE
        -D_WINDOWS -D_USRDLL -D${PROJECT_NAME}_EXPORTS)
endif()