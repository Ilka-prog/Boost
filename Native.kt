package com.example.booster

import android.content.Context

object Native {
    init { System.loadLibrary("booster") }

    /** Завершает фоновые процессы всех не-системных приложений, кроме keep. Возвращает число обработанных или -1. */
    @JvmStatic external fun killAll(ctx: Context, keep: String): Int
}
