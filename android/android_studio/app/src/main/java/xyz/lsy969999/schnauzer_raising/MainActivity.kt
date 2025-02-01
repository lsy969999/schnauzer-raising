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
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.io.InputStream
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

        copyAllFilesToInternal(this);

        Log.d("TAG", "onCreate: ")
        var files = filesDir;


        Log.d("TAG", "onCreate: files: ${files.absolutePath}")
        printInternalFiles(this)
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

fun copyAllAssetsToInternalStorage(context: Context) {
    val assetManager = context.assets
    try {
        val assetFiles = getAssetFiles(assetManager, "")

        for (fileName in assetFiles) {
            copyAssetToInternalStorage(context, fileName)
        }
        println("✅ 모든 assets 파일 복사 완료")
    } catch (e: IOException) {
        e.printStackTrace()
        println("❌ assets 파일 목록 가져오기 실패: ${e.message}")
    }
}

// assets 내부 폴더까지 포함하여 모든 파일 목록 가져오는 함수
fun getAssetFiles(assetManager: android.content.res.AssetManager, path: String): List<String> {
    val fileList = mutableListOf<String>()
    try {
        val files = assetManager.list(path) ?: return emptyList()
        for (file in files) {
            val fullPath = if (path.isEmpty()) file else "$path/$file"
            val subFiles = assetManager.list(fullPath)

            if (subFiles == null || subFiles.isEmpty()) {
                fileList.add(fullPath)  // 파일이면 추가
            } else {
                fileList.addAll(getAssetFiles(assetManager, fullPath))  // 폴더면 재귀 호출
            }
        }
    } catch (e: IOException) {
        e.printStackTrace()
    }
    return fileList
}

// 개별 파일을 내부 저장소로 복사하는 함수
fun copyAssetToInternalStorage(context: Context, assetPath: String) {
    val assetManager = context.assets
    val destFile = File(context.filesDir, assetPath) // 파일명을 전체 경로로 사용

    // 폴더 생성 (내부 저장소에도 동일한 구조 유지)
    destFile.parentFile?.mkdirs()

    // 이미 존재하면 덮어쓰기 방지
    val overwrite = true
    if (!overwrite && destFile.exists()) {
        println("⚠️ 파일 건너뜀: ${destFile.absolutePath}")
        return
    }

    try {
        assetManager.open(assetPath).use { inputStream ->
            FileOutputStream(destFile).use { outputStream ->
                copyStream(inputStream, outputStream)
            }
        }
        println("✅ 파일 복사 성공: ${destFile.absolutePath}")
    } catch (e: IOException) {
        e.printStackTrace()
        println("❌ 파일 복사 실패: ${e.message}")
    }
}

// 스트림 데이터를 복사하는 함수
fun copyStream(input: InputStream, output: FileOutputStream) {
    val buffer = ByteArray(1024)
    var read: Int
    while (input.read(buffer).also { read = it } != -1) {
        output.write(buffer, 0, read)
    }
    output.flush()
}

// 사용 예시
fun copyAllFilesToInternal(context: Context) {
    copyAllAssetsToInternalStorage(context)
}








fun listInternalFiles(context: Context): List<String> {
    val filesDir = context.filesDir
    return listFilesRecursively(filesDir, filesDir.absolutePath)
}

// 재귀적으로 내부 저장소의 모든 파일 & 폴더를 가져오기
fun listFilesRecursively(dir: File, basePath: String): List<String> {
    val fileList = mutableListOf<String>()
    dir.listFiles()?.forEach { file ->
        val relativePath = file.absolutePath.removePrefix("$basePath/")
        fileList.add(relativePath)

        if (file.isDirectory) {
            fileList.addAll(listFilesRecursively(file, basePath)) // 하위 폴더 탐색
        }
    }
    return fileList
}

// 내부 저장소 파일 리스트를 출력하는 함수
fun printInternalFiles(context: Context) {
    val files = listInternalFiles(context)
    if (files.isEmpty()) {
        println("⚠️ 내부 저장소에 파일이 없습니다.")
    } else {
        println("✅ 내부 저장소 파일 목록:")
        files.forEach { println(it) }
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
