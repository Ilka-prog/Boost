package com.example.booster

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.widget.ArrayAdapter
import android.widget.ListView
import android.widget.Toast

class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val main = Intent(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_LAUNCHER)
        val apps = packageManager.queryIntentActivities(main, 0)
            .filter { it.activityInfo.packageName != packageName }
            .map { it.loadLabel(packageManager).toString() to it.activityInfo.packageName }
            .sortedBy { it.first.lowercase() }

        val list = ListView(this)
        list.adapter = ArrayAdapter(this, android.R.layout.simple_list_item_1, apps.map { it.first })
        list.setOnItemClickListener { _, _, pos, _ ->
            val pkg = apps[pos].second
            val n = Native.killAll(this, pkg)
            Toast.makeText(this, if (n >= 0) "Остановлено приложений: $n" else "Ошибка", Toast.LENGTH_SHORT).show()
            packageManager.getLaunchIntentForPackage(pkg)?.let { startActivity(it) }
        }
        setContentView(list)
    }
}
