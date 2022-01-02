package com.example.android

import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.util.Log

import android.webkit.WebView

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        println("Start")
        Log.e("Print", "Start")
        //make a webview object
        val webview= WebView(this)
        webview.getSettings().setJavaScriptEnabled(true)
        setContentView(webview)
        webview.clearCache(true)
        //webview.loadUrl("http://127.0.0.1:8002/")
        webview.loadUrl("file:///android_asset/app/index.html")
        //http://127.0.0.1:8001 / http://10.0.2.2:8089
        Thread(Runnable {
            val rustUrl = hello("")
        }).start()
        println("End")
        //println(hello("Worldddddd rust")) //System.loadLibrary("rust") //Log.e("rust", hello("Worldddddd rust")) //Log.e("Print", rustUrl)
    }

    /*fun printLog(msg: String) {
        Log.e("teste")
    }*/

    external fun hello(to: String): String

    companion object
    {
        // Used to load the 'native-lib' library on application startup.
        init
        {
            System.loadLibrary("rust")
        }
    }

}

