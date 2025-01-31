package xyz.lsy969999.schnauzer_raising

import android.app.NativeActivity
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import java.util.Locale
import java.util.TimeZone

class MainActivity : NativeActivity() {
    private lateinit var spf: SharedPreferences
    private var isSplashKeep: Boolean = true;
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val splashScreen = installSplashScreen()
//        splashScreen.setKeepOnScreenCondition {
//            isSplashKeep
//        }

        spf = getSharedPreferences("kv", Context.MODE_PRIVATE)

        Log.d("TAG", "onCreate: ")
        var files = filesDir;

        Log.d("TAG", "onCreate: files: ${files.absolutePath}")

    }
    fun ffiKvGet(key: String): String {
        val s = spf.getString(key, "");
        return s.toString();
    }
    fun ffiKvSet(key: String, value: String) {
        val editor = spf.edit();
        editor.putString(key, value).apply();
    }

    fun ffiKvDelete(key: String) {
        val editor = spf.edit();
        editor.remove(key).apply();
    }

    fun ffiKvExists(key: String): Boolean {
        return spf.contains(key);
    }


    fun ffiSplashHide() {
        CoroutineScope(Dispatchers.Main).launch {
            delay(500L)  // 0.5초 대기
            isSplashKeep = false
        }
    }

    fun ffiGetCurrentEpochTime(): Long {
        val currentEpochTime = System.currentTimeMillis() / 1000
        return currentEpochTime;
    }

    fun fiiGetLocale(): String {
        val locale = Locale.getDefault()
        val languageCode = locale.language // 언어 코드 (예: ko)
        val countryCode = locale.country
        return "${languageCode}_${countryCode}"
    }

    fun ffiGetTimeOffset(): Int {
        val timeZone = TimeZone.getDefault()
        val offsetInMillis = timeZone.rawOffset  // UTC와의 시간 오프셋 (밀리초 단위)
        val offsetInSecs = offsetInMillis / 1000    // 시간 단위로 변환
//        print("offsetInSec: $offsetInSecs")
        return offsetInSecs
    }

}

class RustBinding {
//    companion object {
//        init {
//            System.loadLibrary("android");
//        }

//        @JvmStatic
//        external fun ffi_callback_admob_interstitial_load_success();
//        @JvmStatic
//        external fun ffi_callback_admob_interstitial_load_fail(errMsg: String);
//        @JvmStatic
//        external fun ffi_callback_admob_interstitial_showed();
//        @JvmStatic
//        external fun ffi_callback_admob_interstitial_show_fail(errMsg: String);
//        @JvmStatic
//        external fun ffi_callback_admob_interstitial_dismissed();
//
//        @JvmStatic
//        external fun ffi_callback_admob_interstitial_is_ready(isReady: Boolean);
//
//        @JvmStatic
//        external fun ffi_callback_app_init_end();
//    }
}
