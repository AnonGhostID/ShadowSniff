/*
 * This file is part of ShadowSniff (https://github.com/sqlerrorthing/ShadowSniff)
 *
 * MIT License
 *
 * Copyright (c) 2025 sqlerrorthing
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

fn main() {
    cc::Build::new()
        .file("sqlite3/sqlite3.c")
        .define("SQLITE_OMIT_LOAD_EXTENSION", None)
        .define("SQLITE_THREADSAFE", "0")
        .define("SQLITE_OMIT_UTF16", None)
        .define("SQLITE_OMIT_AUTOINIT", None)
        .define("SQLITE_OMIT_DEPRECATED", None)
        .define("SQLITE_OMIT_TRACE", None)
        .define("SQLITE_OMIT_PROGRESS_CALLBACK", None)
        .define("SQLITE_OMIT_SHARED_CACHE", None)
        .define("SQLITE_OMIT_WAL", None)
        .define("SQLITE_OMIT_JSON", None)
        .define("SQLITE_OMIT_COMPLETE", None)
        .define("SQLITE_OMIT_TCL_VARIABLE", None)
        .define("SQLITE_OMIT_TCL", None)
        .define("SQLITE_TEMP_STORE", "0")
        .define("SQLITE_DQS", "0")
        .define("SQLITE_OMIT_TXN_STATE", None)
        .define("SQLITE_OMIT_BLOB_LITERAL", None)
        .define("SQLITE_OMIT_AUTHORIZATION", None)
        .define("SQLITE_DEFAULT_MEMSTATUS", "0")
        .define("SQLITE_OMIT_FOREIGN_KEY", None)
        .define("SQLITE_OMIT_SCHEMA_PRAGMAS", None)
        .define("SQLITE_OMIT_EXPLAIN", None)
        .define("SQLITE_OMIT_COMPLETE", None)
        .define("SQLITE_OMIT_COMPILEOPTION_DIAGS", None)
        .define("SQLITE_OMIT_UPDATE_DELETE_LIMIT", None)
        .define("SQLITE_OMIT_DECLTYPE", None)
        .define("SQLITE_LIKE_DOESNT_MATCH_BLOBS", None)
        .define("SQLITE_OMIT_WRITEONLY_CONVERSIONS", None)
        .define("SQLITE_OMIT_AUTOVACUUM", None)
        .define("SQLITE_OMIT_AUTOINCREMENT", None)
        .define("SQLITE_OMIT_AUTOMATIC_INDEX", None)
        .define("SQLITE_OMIT_AUTORESET", None)
        .define("SQLITE_OMIT_CAST", None)
        .define("SQLITE_OMIT_CHECK", None)
        .define("SQLITE_OMIT_COMPOUND_SELECT", None)
        .define("SQLITE_OMIT_DATETIME_FUNCS", None)
        .define("SQLITE_OMIT_DEPRECATED", None)
        .define("SQLITE_OMIT_FLAG_PRAGMAS", None)
        .define("SQLITE_OMIT_GENERATED_COLUMNS", None)
        .define("SQLITE_OMIT_GET_TABLE", None)
        .define("SQLITE_OMIT_INCRBLOB", None)
        .define("SQLITE_OMIT_INTEGRITY_CHECK", None)
        .define("SQLITE_OMIT_INTROSPECTION_PRAGMAS", None)
        .define("SQLITE_OMIT_LIKE_OPTIMIZATION", None)
        .define("SQLITE_OMIT_LOCALTIME", None)
        .define("SQLITE_OMIT_LOOKASIDE", None)
        .define("SQLITE_OMIT_OR_OPTIMIZATION", None)
        .define("SQLITE_OMIT_PROGRESS_CALLBACK", None)
        .define("SQLITE_OMIT_TCL_VARIABLE", None)
        .define("SQLITE_OMIT_TEMPDB", None)
        .define("SQLITE_OMIT_TRACE", None)
        .define("SQLITE_UNTESTABLE", None)
        .define("SQLITE_DISABLE_PAGECACHE_OVERFLOW_STATS", None)
        .define("SQLITE_DISABLE_DIRSYNC", None)
        .define("SQLITE_OMIT_BETWEEN_OPTIMIZATION", None)
        .define("SQLITE_OMIT_CASE_SENSITIVE_LIKE_PRAGMA", None)
        .opt_level(1)
        .flag("/DNDEBUG")
        .flag("/EHsc-")
        .flag("/GR-")
        .flag("/GF")
        .flag("/GS-")
        .flag("/Zl")
        .flag("/Gw")
        .flag("/Gy")
        .compile("sqlite3");

    println!("cargo:rustc-link-lib=static=sqlite3");
}
