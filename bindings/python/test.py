
import os
import ctypes

lib_name = "libebpf_toolchain_cdylib.so"
lib_dir = "../c/target/debug"
lib_path = os.path.join(lib_dir, lib_name)

print("Attempting to load:\n'{}'".format(lib_path))
ebpf_toolchain = ctypes.cdll.LoadLibrary(lib_path)

print("Trying to run ...")
r = ebpf_toolchain.try_this()

print("r = {}".format(r))
