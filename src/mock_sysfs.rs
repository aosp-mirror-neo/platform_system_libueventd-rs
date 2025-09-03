// Copyright (C) 2025 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::env::temp_dir;
use std::fs;
use std::io;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Enum representing different types of files.
pub enum SysfsFile {
    /// Directory.
    Dir(HashMap<&'static str, SysfsFile>),
    /// Regular file.
    RegularFile(&'static str),
    /// Symbolic link.
    Symlink(&'static str),
}

/// Structure containing a mock sysfs tree.
pub struct MockSysfs {
    root: PathBuf,
}

impl MockSysfs {
    /// Instantiates the file tree described by `sysfs` in a temporary directory
    /// and returns the object representing it.
    ///
    /// This directory will be removed when the `MockSysfs` object is dropped.
    pub fn new(sysfs: SysfsFile) -> io::Result<MockSysfs> {
        let mut temp_root = temp_dir();
        temp_root.push(format!(
            "libueventd_unittest_sys_{}_{:?}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(),
            std::thread::current().id()
        ));

        let res = MockSysfs { root: temp_root };
        Self::create_file(res.path(), &sysfs)?;
        Ok(res)
    }

    /// Gets the path to the root of the mock sysfs tree.
    pub fn path(&self) -> &Path {
        self.root.as_path()
    }

    fn create_file<P: AsRef<Path>>(path: P, file: &SysfsFile) -> io::Result<()> {
        match file {
            SysfsFile::Dir(contents) => {
                fs::create_dir_all(&path)?;
                for (child_name, child_file) in contents {
                    let child_path = path.as_ref().join(child_name);
                    Self::create_file(&child_path, child_file)?;
                }
                Ok(())
            }
            SysfsFile::RegularFile(contents) => fs::write(path, contents),
            SysfsFile::Symlink(contents) => symlink(contents, path),
        }
    }
}

impl Drop for MockSysfs {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(self.path());
    }
}
