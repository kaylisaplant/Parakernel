cmake_minimum_required(VERSION 3.16)

project(IParaViewKernel)

find_package(Python3 COMPONENTS Interpreter REQUIRED)

add_custom_target(IParaViewKernelTarget ALL
  COMMAND
    ${CMAKE_COMMAND}
      # In and out directory
      -DIN_DIRECTORY=${CMAKE_CURRENT_SOURCE_DIR}
      -DOUT_DIRECTORY=${CMAKE_CURRENT_BINARY_DIR}
      # Pass needed CMake variables for the configure_file function
      -DParaView_PREFIX_PATH=${ParaView_PREFIX_PATH}
      -DPARAVIEW_PYTHONPATH=${PARAVIEW_PYTHONPATH}
      -DPython3_EXECUTABLE=${Python3_EXECUTABLE}
      -DCMAKE_INSTALL_PREFIX=${CMAKE_INSTALL_PREFIX}
      -DCMAKE_PROJECT_NAME=${CMAKE_PROJECT_NAME}
      # CMake script launching configure_file
      -P ${CMAKE_CURRENT_SOURCE_DIR}/configure_files.cmake
  BYPRODUCTS
    ${CMAKE_CURRENT_BINARY_DIR}/kernel.json
    ${CMAKE_CURRENT_BINARY_DIR}/config.py
  DEPENDS
    kernel.json.in
    config.py.in
  VERBATIM)

install(FILES
    paraview_kernel.py
    ${CMAKE_CURRENT_BINARY_DIR}/kernel.json
    ${CMAKE_CURRENT_BINARY_DIR}/config.py
  DESTINATION
    ${CMAKE_PROJECT_NAME})
