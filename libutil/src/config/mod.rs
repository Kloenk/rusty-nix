/*
 * This file was part of nix_cfg, a parser for the Nix configuration format.
 * now adapted to libutil a general nix util library
 * Copyright © 2020 Milan Pässler
 * Copyright © 2020 Finn Behrens
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod error;

use crate::config::error::{Error, Result};
use log::{trace, warn};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{de, forward_to_deserialize_any, Deserialize, Serialize};
use std::ops::{AddAssign, MulAssign, Neg};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct NixConfig {
    #[serde(default = "default_store")]
    store: String, // The default Nix store to use.

    keep_failed: bool, // Whether to keep temporary directories of failed builds.
    keep_going: bool,  // Whether to keep building derivations when another build fails.
    #[serde(alias = "build-fallback")]
    fallback: bool, // Whether to fall back to building when substitution fails.

    #[serde(default = "default_true")]
    verbose_build: bool, // Whether to show build log output in real time.

    #[serde(default = "default_ten")]
    log_lines: usize,

    #[serde(default = "default_max_jobs", alias = "build-max-jobs")]
    max_jobs: String, // Maximum number of parallel build jobs.

    #[serde(default = "default_cores", alias = "build-cores")]
    cores: usize, // umber of CPU cores to utilize in parallel within a build, i.e. by passing this number to Make via '-j'. 0 means that the number of actual CPU cores on the local host ought to be auto-detected

    read_only: bool,

    #[serde(default = "default_system")]
    system: String, // The canonical Nix system name.

    #[serde(alias = "build-max-silent-time")]
    max_silent_time: usize, // The maximum time in seconds that a builer can go without producing any output on stdout/stderr before it is killed. 0 means infinity.
    #[serde(alias = "build-timeout")]
    timeout: usize, // The maximum duration in seconds that a builder can run. 0 means infinity.

    #[serde(default = "default_build_hook")]
    build_hook: String, // The path of the helper program that executes builds to remote machines.

    // FIXME: default value
    builders: String, // A semicolon-separated list of build machines, in the format of nix.machines.
    builders_use_substitutes: bool, // Whether build machines should use their own substitutes for obtaining build dependencies if possible, rather than waiting for this host to upload them.

    // FIXME: default value
    gc_reserved_space: usize, // Amount of reserved disk space for the garbage collector.

    #[serde(default = "default_true")]
    fsync_metdata: bool, // "Amount of reserved disk space for the garbage collector.

    #[serde(default = "default_true")] // FIXME: not on WSL1
    use_sqlite_wal: bool,

    sync_before_registering: bool, // Whether to call sync() before registering a path as valid.

    #[serde(default = "default_true", alias = "build-use-substitutes")]
    substitute: bool, // Whether to use substitutes.

    build_users_group: String, // The Unix group that contains the build users.

    #[serde(alias = "build-impersonate-linux-26")]
    impersonate_linux_26: bool, // Whether to impersonate a Linux 2.6 machine on newer kernels.

    #[serde(default = "default_true", alias = "build-keep-log")]
    keep_build_log: bool, // Whether to store build logs.

    #[serde(default = "default_true", alias = "build-compress-log")]
    compress_build_log: bool, // Whether to compress logs.

    #[serde(alias = "build-max-log-size")]
    max_buid_log_size: usize, // Maximum number of bytes a builder can write to stdout/stderr before being killed (0 means no limit).

    #[serde(default = "default_ten")]
    build_poll_interval: usize, // How often (in seconds) to poll for locks.

    gc_check_reachability: bool, // Whether to check if new GC roots can in fact be found by the garbage collector.

    #[serde(alias = "keep-outputs")]
    gc_keep_outputs: bool, // Whether the garbage collector should keep outputs of live derivations.

    #[serde(default = "default_true", alias = "keep-derivations")]
    gc_keep_derivations: bool, // Whether the garbage collector should keep derivers of live paths.

    auto_optimise_store: bool, // Whether to automatically replace files with identical contents with hard links.

    #[serde(alias = "env-keep-derivations")]
    keep_env_derivations: bool, // Whether to add derivations as a dependency of user environments (to prevent them from being GCed).

    show_trace: bool, // Whether to show a stack trace on evaluation errors.

    // FIXME: default value
    #[serde(alias = "build-use-chroot", alias = "build-use-sandbox")]
    sandbox: String, // Whether to enable sandboxed builds. Can be \"true\", \"false\" or \"relaxed\".

    #[serde(alias = "build-chroot-dirs", alias = "build-sandbox-paths")]
    sandbox_paths: Vec<String>, // The paths to make available inside the build sandbox.

    #[serde(default = "default_true")]
    sandbox_fallback: bool, // Whether to disable sandboxing when the kernel doesn't allow it.

    #[serde(alias = "build-extra-chroot-dirs", alias = "build-extra-sandbox-paths")]
    extra_sandbox_pathes: Vec<String>, // Additional paths to make available inside the build sandbox.

    #[serde(alias = "repeat")]
    build_repeat: usize, // The number of times to repeat a build in order to verify determinism.

    #[cfg(target_os = "linux")]
    #[serde(default = "default_sandbox_dev_shm_size")]
    sandbox_dev_shm_size: String, // The size of /dev/shm in the build sandbox.

    #[cfg(target_os = "linux")]
    #[serde(default = "default_sandbox_build_dir")]
    sandbox_build_dir: String, // The build directory inside the sandbox.

    allowed_impure_host_deps: Vec<String>, // Which prefixes to allow derivations to ask for access to (primarily for Darwin).

    #[cfg(target_os = "macos")]
    darwin_log_sandbox_violations: bool, // Whether to log Darwin sandbox access violations to the system log.

    run_diff_hook: bool, // Whether to run the program specified by the diff-hook setting repeated builds produce a different result. Typically used to plug in diffoscope.

    diff_hook: String, // A program that prints out the differences between the two paths specified on its command line.

    #[serde(default = "default_true")]
    enforce_determinism: bool, // Whether to fail if repeated builds produce different output.

    #[serde(
        default = "default_trusted_pub_keys",
        alias = "binary-cache-public-keys"
    )]
    trusted_public_keys: Vec<String>, // Trusted public keys for secure substitution.

    secret_key_files: Vec<String>, // Secret keys with which to sign local builds.

    #[serde(default = "default_tarball_ttl")]
    tarball_ttl: usize, // How long downloaded files are considered up-to-date.

    #[serde(default = "default_true")]
    require_sigs: bool, // Whether to check that any non-content-addressed path added to the Nix store has a valid signature (that is, one signed using a key listed in 'trusted-public-keys'.

    // FIXME: i686-linux if we are x86_64-linux
    extra_platforms: Vec<String>, // Additional platforms that can be built on the local system. These may be supported natively (e.g. armv7 on some aarch64 CPUs or using hacks like qemu-user.

    // FIXME: getDefaultSystemFeatures()
    system_features: Vec<String>, // Optional features that this system implements (like \"kvm\").

    #[serde(default = "default_substiturers")]
    substituters: Vec<String>, // The URIs of substituters (such as https://cache.nixos.org/).

    extra_substituters: Vec<String>, // Additional URIs of substituters.

    trusted_substituters: Vec<String>, // Disabled substituters that may be enabled via the substituters option by untrusted users.

    #[serde(default = "default_trusted_users")]
    trusted_users: Vec<String>, // Which users or groups are trusted to ask the daemon to do unsafe things.

    #[serde(default = "default_narinfo_cache_negative_ttl")]
    narinfo_cache_negative_ttl: usize, // The TTL in seconds for negative lookups in the disk cache i.e binary cache lookups that return an invalid path result

    #[serde(default = "default_narinfo_cache_positive_ttl")]
    narinfo_cache_positive_ttl: usize, // The TTL in seconds for positive lookups in the disk cache i.e binary cache lookups that return a valid path result.

    #[serde(default = "default_allowed_users")]
    allowed_users: Vec<String>, // Which users or groups are allowed to connect to the daemon.

    #[serde(default = "default_true")]
    print_missing: bool, // Whether to print what paths need to be built or downloaded.

    pre_build_hook: String, // A program to run just before a build to set derivation-specific build settings."

    post_build_hook: String, // A program to run just after each successful build.

    // FIXME: default value
    netrc: String, // Path to the netrc file used to obtain usernames/passwords for downloads.

    // caFile // Path to the SSL CA file used
    #[cfg(target_os = "linux")]
    #[serde(default = "default_true")]
    filter_syscalls: bool, // Whether to prevent certain dangerous system calls, such as creation of setuid/setgid files or adding ACLs or extended attributes. Only disable this if you're aware of the security implications.

    #[cfg(target_os = "linux")]
    allow_new_privileges: bool, // Whether builders can acquire new privileges by calling programs with setuid/setgid bits or with file capabilities.

    #[serde(default = "default_hashed_mirrors")]
    hashed_mirrors: Vec<String>, // A list of servers used by builtins.fetchurl to fetch files by hash.

    min_free: usize, // Automatically run the garbage collector when free disk space drops below the specified amount.

    // FIXME: usize::max as default
    max_free: usize, // Stop deleting garbage when free disk space is above the specified amount.

    #[serde(default = "default_min_free_checking_intervall")]
    min_free_check_interval: usize, // Number of seconds between checking free disk space.

    plugin_files: Vec<String>, // warn if used!!!

    github_access_token: String,

    experimenat_features: Vec<String>, // Experimental Nix features to enable.

    #[serde(default = "default_true")]
    allow_dirty: bool, // Whether to allow dirty Git/Mercurial trees.

    #[serde(default = "default_true")]
    warn_dirty: bool, // Whether to warn about dirty Git/Mercurial trees.

    #[serde(default = "default_flake_registries")]
    flake_registry: String, // Path or URI of the global flake registry.
}

impl NixConfig {
    pub fn parse_file(file: &std::path::Path) -> Result<Self> {
        let old_dir = std::env::current_dir().unwrap();
        let base_path = file.parent().unwrap();
        std::env::set_current_dir(&base_path).unwrap();

        let config_text = std::fs::read_to_string(file).unwrap();
        let config_text = Self::pre_text(config_text)?;
        let config: Result<NixConfig> = crate::config::from_str(&config_text);

        std::env::set_current_dir(&old_dir.as_path()).unwrap();

        // give warings
        config
            .as_ref()
            .map(|v| {
                for v in &v.plugin_files {
                    eprintln!(
                        "Warning: the plugin {} could not be loaded, because we are running rust",
                        v
                    )
                }
            })
            .unwrap(); // FIXME: find something better to do with Err

        config
    }

    pub fn pre_text(text: String) -> Result<String> {
        let mut end_text = String::new();
        for line in text.lines() {
            if line.starts_with('#') {
            } else if line.is_empty() {
            } else if line.starts_with("include") {
                // TODO include
            } else if line.starts_with("!include") {
                // TODO try include
            } else {
                // TODO parse commands at the end
                end_text.push_str(&format!("{}\n", line));
            }
        }
        Ok(end_text)
    }
}

fn default_store() -> String {
    String::from("auto") // TODO: read from env
}

fn default_max_jobs() -> String {
    String::from("1")
}

fn default_cores() -> usize {
    8 // TODO: get core number
}

fn default_system() -> String {
    String::from("x86_64-linux") // TODO: get system type
}

fn default_build_hook() -> String {
    String::from("/run/current-system/sw/bin/nix/build-remote") // FIXME: add path
}

fn default_trusted_pub_keys() -> Vec<String> {
    vec![
        String::from("cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="), // TODO: parse from build.rs as build time config?
    ]
}

fn default_substiturers() -> Vec<String> {
    vec![
        String::from("https://cache.nixos.org/"), // TODO: only if nixStore is /nix/store
    ]
}

fn default_hashed_mirrors() -> Vec<String> {
    vec![String::from("http://tarballs.nixos.org/")]
}

fn default_flake_registries() -> String {
    String::from("https://github.com/NixOS/flake-registry/raw/master/flake-registry.json")
}

fn default_trusted_users() -> Vec<String> {
    vec![String::from("root")]
}

fn default_allowed_users() -> Vec<String> {
    vec![String::from("*")]
}

fn default_tarball_ttl() -> usize {
    60 * 60
}

fn default_true() -> bool {
    true
}

fn default_ten() -> usize {
    10
}

fn default_narinfo_cache_negative_ttl() -> usize {
    3600
}

fn default_narinfo_cache_positive_ttl() -> usize {
    30 * 24 * 3600
}

fn default_min_free_checking_intervall() -> usize {
    5
}

#[cfg(target_os = "linux")]
fn default_sandbox_dev_shm_size() -> String {
    String::from("50%")
}

#[cfg(target_os = "linux")]
fn default_sandbox_build_dir() -> String {
    String::from("/build")
}

struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        // FIXME: handle escape sequences and/or quoting
        match self.input.find(char::is_whitespace) {
            Some(len) => {
                trace!("len: {}", len);
                let s = &self.input[..len];
                self.input = &self.input[len..];
                trace!("parsed as string: {}", s);
                Ok(s)
            }
            None => Err(Error::Eof),
        }
    }

    fn parse_bool(&mut self) -> Result<bool> {
        if self.input.starts_with("true") {
            self.input = &self.input["true".len()..];
            return Ok(true.into());
        }
        if self.input.starts_with("false") {
            self.input = &self.input["false".len()..];
            return Ok(false.into());
        }
        Err(Error::ExpectedBool)
    }

    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        let mut int = match self.input.chars().next().ok_or(Error::Eof)? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
    {
        unimplemented!()
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de> MapAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.input = self.input.trim_start_matches("\n");
        if self.input.is_empty() {
            return Ok(None);
        }
        seed.deserialize(self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.input.starts_with(" =\n") {
            // FIXME: reset value
            self.input = &self.input[" =\n".len()..];
        } else if !self.input.starts_with(" = ") {
            trace!("parsed until here:\n{}", self.input);
            return Err(Error::ExpectedMapEquals);
        }
        self.input = &self.input[" = ".len()..];
        seed.deserialize(self)
    }
}

impl<'de> SeqAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.input.starts_with("\n") {
            return Ok(None);
        }
        self.input = self.input.trim_start_matches(" ");
        seed.deserialize(self).map(Some)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Ok(visitor.visit_seq(self)?)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Ok(visitor.visit_map(self)?)
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Ok(visitor.visit_map(self)?)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.input.find('\n').ok_or(Error::Eof)?;
        warn!("unknown option with value \"{}\"", &self.input[..len]);
        self.input = &self.input[len..];
        Ok(visitor.visit_none()?)
    }

    forward_to_deserialize_any! {
        tuple bytes byte_buf option unit unit_struct newtype_struct tuple_struct enum f32 f64
    }
}
