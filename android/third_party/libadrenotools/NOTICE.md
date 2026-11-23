# libadrenotools in this repository

- libadrenotools from <https://github.com/bylaws/libadrenotools>, commit
  `8fae8ce254dfc1344527e05301e43f37dea2df80`, without its `tools/` folder.
- liblinkernsbypass (`lib/linkernsbypass`, a submodule upstream) from
  <https://github.com/bylaws/liblinkernsbypass>, commit
  `aa3975893d83ef1bc84c321ec60c65fbf1287887`.

Both are Copyright (c) 2021 Billy Laws, under the BSD 2-Clause License: the
full texts are `LICENSE` here and `lib/linkernsbypass/LICENSE`. The files are
unchanged.

`android/build.ps1` builds `libadrenotools.so`, `libhook_impl.so` and
`libmain_hook.so` from them with the Android SDK's CMake and the NDK, and packs
them into the APK as assets (`gpu_hooks/`). The app unpacks them only when the
player picks a GPU driver (`GpuDrivers.java`, `crates/android/src/gpu_driver.rs`).
