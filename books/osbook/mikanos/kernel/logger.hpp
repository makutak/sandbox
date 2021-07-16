/**
 * @file logger.hpp
 *
 * カーネルロガーの実装
 */

#pragma once

enum LogLevel {
  kError = 3,
  kWarn = 4,
  kInfo = 6,
  kDebug = 7,
};

/*
 * グローバルなログ優先度の閾値を level に設定する。
 * 以降の Log 呼び出しでは、ここで設定された優先度以上のログのみ記録される。
 */
void SetLogLevel(LogLevel level);


/*
 * 指定された優先度が閾値以上ならば記録する。
 * 優先度が閾値未満ならログは捨てられる。
 *
 * @param level ログの優先度。閾値以上の優先度のログのみが記録される。
 * @param format 書式文字列。printk と互換。
 */
int Log(LogLevel level, const char* format, ...);
