package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.assertFalse;

import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;

import org.junit.jupiter.api.Test;

/**
 * The fixed, invocation-independent store path (CLAUDE.md → AUTH), via the injected env
 * lookup — never {@code System.getenv} mutation. Both platform branches are plain
 * (non-{@code cfg}) Java, so both are exercised on every CI OS.
 */
class ConfigDirTest {

    private static Map<String, String> env(String... pairs) {
        Map<String, String> map = new HashMap<>();
        for (int i = 0; i < pairs.length; i += 2) {
            map.put(pairs[i], pairs[i + 1]);
        }
        return map;
    }

    @Test
    void prefersXdgConfigHome() throws Exception {
        assertEquals(
                Paths.get("/xdg", "oura-toolkit"),
                TokenStore.configDir(env("XDG_CONFIG_HOME", "/xdg", "HOME", "/home/u")::get, false));
    }

    @Test
    void fallsBackToHomeDotConfig() throws Exception {
        assertEquals(
                Paths.get("/home/u", ".config", "oura-toolkit"),
                TokenStore.configDir(env("HOME", "/home/u")::get, false));
    }

    @Test
    void emptyOrRelativeXdgFallsBackToHome() throws Exception {
        for (String bad : new String[] {"", "relative/config"}) {
            assertEquals(
                    Paths.get("/home/u", ".config", "oura-toolkit"),
                    TokenStore.configDir(
                            env("XDG_CONFIG_HOME", bad, "HOME", "/home/u")::get, false),
                    "XDG_CONFIG_HOME=" + bad + " must be ignored "
                            + "(a relative base would make secret placement cwd-dependent)");
        }
    }

    @Test
    void emptyOrRelativeHomeErrors() {
        for (String bad : new String[] {"", "relative/home"}) {
            NoConfigDirException e = assertThrows(
                    NoConfigDirException.class,
                    () -> TokenStore.configDir(env("HOME", bad)::get, false),
                    "HOME=" + bad + " must not resolve");
            assertTrue(e.getMessage().contains("$XDG_CONFIG_HOME"),
                    "Unix error must name the Unix env vars: " + e.getMessage());
        }
    }

    @Test
    void errorsWhenNothingIsSet() {
        assertThrows(
                NoConfigDirException.class,
                () -> TokenStore.configDir(env()::get, false));
    }

    @Test
    void windowsUsesLocalAppdataNotRoaming() throws Exception {
        assertEquals(
                Paths.get("C:\\Users\\u\\AppData\\Local", "oura-toolkit"),
                TokenStore.configDir(
                        env(
                                "LOCALAPPDATA", "C:\\Users\\u\\AppData\\Local",
                                "APPDATA", "C:\\Users\\u\\AppData\\Roaming")::get,
                        true),
                "must use machine-local %LOCALAPPDATA%, never the roaming profile "
                        + "(roaming syncs plaintext secrets off the machine)");
    }

    @Test
    void windowsEmptyOrRelativeLocalAppdataErrors() {
        for (String bad : new String[] {"", "relative\\path"}) {
            NoConfigDirException e = assertThrows(
                    NoConfigDirException.class,
                    () -> TokenStore.configDir(env("LOCALAPPDATA", bad)::get, true),
                    "LOCALAPPDATA=" + bad + " must not resolve");
            assertTrue(e.getMessage().contains("%LOCALAPPDATA%"),
                    "Windows users must not be told about Unix env vars: " + e.getMessage());
            assertFalse(e.getMessage().contains("XDG"),
                    "Windows error must not mention XDG: " + e.getMessage());
        }
    }

    @Test
    void windowsAcceptsUncPaths() throws Exception {
        assertEquals(
                Paths.get("\\\\server\\share\\AppData\\Local", "oura-toolkit"),
                TokenStore.configDir(
                        env("LOCALAPPDATA", "\\\\server\\share\\AppData\\Local")::get, true));
    }
}
